use anyhow::anyhow;
use base_util::onnx::{dyn_to_2d, new_session};
use interface::{
    detectors::{Detector, Mask, textlines::Quadrilateral},
    image::{DimType, ImageOp, Interpolation, RawImage},
    model::{CreateData, Model, ModelSource},
};
use log::debug;

use ndarray::{Array, Array3, ArrayViewD, Axis, Dim, IxDynImpl, array};
use opencv::core::BORDER_DEFAULT;
use ort::{session::Session, value::Tensor};
use util::{dbnet::Batch, opencv::bilateral_filter};

use maplit::hashmap;

pub struct DbNetDetector {
    db: CreateData,
    model: Option<Session>,
    /// Different model architecture, but based on dbnet
    convnext: bool,
}

impl DbNetDetector {
    ///convnext: Different model architecture, but based on dbnet
    pub fn new(db: CreateData, convnext: bool) -> Self {
        DbNetDetector {
            db,
            model: None,
            convnext,
        }
    }
}

impl Model for DbNetDetector {
    fn models(&self) -> std::collections::HashMap<&'static str, ModelSource> {
        hashmap! {
            "model"=> ModelSource {
                url:"https://github.com/frederik-uni/manga-image-translator-rust/releases/download/detect-default-20241225.onnx",
                hash:"7b348114b09015ce18373049c0ff90ce9a55fd3378cd33fd6209c80d1d04660e",
                file: None,
                archive: None
            }
        }
    }

    fn loaded(&self) -> bool {
        self.model.is_some()
    }

    fn unload(&mut self) {
        self.model = None;
    }

    fn load(&mut self) -> anyhow::Result<()> {
        let models = self.models();
        let models = models.get("model").expect("Modelname was registered");
        self.model = Some(new_session(
            self.db
                .mode_db
                .get(
                    self.kind(),
                    self.name(),
                    "model.onnx",
                    models.url,
                    models.hash,
                )
                .ok_or(anyhow!("No model found"))?,
            self.db.providers.clone(),
        )?);
        Ok(())
    }

    fn name(&self) -> &'static str {
        "dbnet"
    }

    fn kind(&self) -> &'static str {
        "detector"
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct DefaultOptions {
    /// Text detector used for creating a text mask from an image
    /// TODO: guide
    pub detect_size: u64,
    /// How much to extend text skeleton to form bounding box
    /// smaller values = smaller text skeleton.
    /// to small = more false negatives/partial detections
    /// larger values = bigger text skeleton detections .
    /// to big =  more false positives/Multiple close text lines/words may be merged
    /// Suggested values:
    /// - `1.0 – 1.5`: Use for tight text layouts, well-separated characters or lines, high-resolution images.
    /// - `1.5 – 2.0`: General-purpose setting. Provides a good balance between recall and precision.
    /// - `2.0 – 2.5`: Use when text is thin, faint, or sparse—e.g., scanned documents or light fonts.
    /// - `> 2.5`: Rarely needed. May cause nearby text instances to merge or overlap.
    pub unclip_ratio: f64,
    /// Threshold for text detection
    /// smaller values = more detections + more false positives
    /// larger values = fewer detections + more false negatives
    /// allowed range is from 0.0 to 1.0
    pub text_threshold: f64,
    /// Threshold for bbox generation
    /// to small = more false positives/ noise, background artifacts, or partial text.
    /// to big = false negatives/ actual text that had slightly lower confidence is discarded.
    /// allowed range is from 0.0 to 1.0
    pub box_threshold: f64,
}

impl DefaultOptions {
    pub fn dump(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                (self as *const Self) as *const u8,
                std::mem::size_of::<Self>(),
            )
        }
    }

    fn parse(options: &[u8]) -> anyhow::Result<Self> {
        if options.len() < std::mem::size_of::<Self>() {
            anyhow::bail!("Options too small");
        }

        let ptr = options.as_ptr() as *const Self;

        let config = unsafe { &*ptr };

        Ok(config.to_owned())
    }
}

impl Default for DefaultOptions {
    fn default() -> Self {
        Self {
            detect_size: 2048,
            unclip_ratio: 2.3,
            text_threshold: 0.5,
            box_threshold: 0.7,
        }
    }
}

impl Detector for DbNetDetector {
    fn infer(
        &mut self,
        detect: RawImage,
        options: &[u8],
        img_processor: &Box<dyn ImageOp + Send + Sync>,
    ) -> anyhow::Result<(Vec<Quadrilateral>, Mask)> {
        let options = DefaultOptions::parse(options)?;
        let session = match &mut self.model {
            None => {
                self.load()?;
                self.model.as_mut().expect("Model should be loaded")
            }
            Some(model) => model,
        };
        let resized = util::imageproc::resize_aspect_ratio(
            bilateral_filter(&detect.as_opencv_mat(), 17, 80.0, 80.0, BORDER_DEFAULT)?,
            options.detect_size as i64,
            Interpolation::Bilinear,
            1.0,
            img_processor,
        );
        let ratio_h = 1.0 / resized.ratio;
        let ratio_w = ratio_h;
        let shape = (resized.img.height, resized.img.width);

        let array = resized
            .img
            .to_ndarray()
            .mapv(|x| x as f32 / 127.5 - 1.0)
            .permuted_axes([2, 0, 1])
            .insert_axis(ndarray::Axis(0));

        let tensor = Tensor::from_array(array).unwrap();
        let outputs = session.run(ort::inputs!["input" => tensor]).unwrap();
        let db: ArrayViewD<f32> = outputs["db"].try_extract_array().unwrap();
        let mask: ArrayViewD<f32> = outputs["mask"].try_extract_array().unwrap();
        let db = db.mapv(|x| 1.0 / (1.0 + (-x).exp()));

        let mask: Array<f32, Dim<IxDynImpl>> = mask
            .index_axis(ndarray::Axis(0), 0)
            .index_axis(ndarray::Axis(0), 0)
            .to_owned();

        debug!("Detection resolution: {}x{}", shape.1, shape.0);

        let det = util::dbnet::SegDetectorRepresenter {
            min_size: 3.0,
            thresh: options.text_threshold as f32,
            box_thresh: options.box_threshold,
            max_candidates: 1000,
            unclip_ratio: options.unclip_ratio,
        };
        let mask = dyn_to_2d(mask).unwrap();

        let (mut boxes, mut scores) = det.call(
            Batch { shape: vec![shape] },
            db.mapv(|v| v).into_dimensionality().unwrap(),
            false,
        );

        let boxes = boxes.remove(0).unwrap();
        let scores = scores.remove(0).unwrap();
        let polys = filter_boxes_and_adjust(&boxes, ratio_w, ratio_h);
        let quadrilateral = polys
            .outer_iter()
            .zip(scores)
            .map(|(pts, score)| {
                Quadrilateral::new(
                    pts.outer_iter()
                        .map(|v| (v[0], v[1]))
                        .collect::<Vec<(i64, i64)>>(),
                    score,
                )
            })
            .filter(|v| v.area() >= 16.0)
            .collect::<Vec<_>>();

        let mask_shape = mask.shape();
        let mask_width = mask_shape[1] * 2;
        let mask_height = mask_shape[0] * 2;
        let mask = mask
            .mapv(|v| f32::clamp(v * 255.0, 0.0, 255.0) as u8)
            .into_iter()
            .collect::<Vec<u8>>();
        let mask_resized = img_processor.resize_mask(
            mask,
            mask_shape[1],
            mask_shape[0],
            mask_width,
            mask_height,
            Interpolation::Bilinear,
        );
        let new_mask_width = mask_width - resized.pad_w as usize;
        let new_mask_height = mask_height - resized.pad_h as usize;
        let mut mask_resized = Mask {
            width: mask_width as DimType,
            height: mask_height as DimType,
            data: mask_resized,
        };
        if resized.pad_h > 0 || resized.pad_w > 0 {
            mask_resized = img_processor.remove_border_mask(
                mask_resized,
                new_mask_width as DimType,
                new_mask_height as DimType,
            );
        }

        Ok((quadrilateral, mask_resized))
    }
}

fn filter_boxes_and_adjust(boxes: &Array3<i64>, ratio_w: f64, ratio_h: f64) -> Array3<i64> {
    if boxes.is_empty() {
        return Array3::<i64>::zeros((0, 0, 0));
    }
    let boxes = boxes.to_shared();
    let idx = boxes
        .reshape((boxes.shape()[0], boxes.len() / boxes.shape()[0]))
        .sum_axis(Axis(1))
        .mapv(|v| v > 0);
    let indicies = idx
        .iter()
        .enumerate()
        .filter(|(_, b)| **b)
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();
    let polys = boxes.select(Axis(0), &indicies);
    let polys = polys.mapv(|v| v as f64);
    let polys = adjust_result_coordinates(polys, ratio_w, ratio_h, 1.0);
    polys.mapv(|v| v as i64)
}

fn adjust_result_coordinates(
    polys: Array3<f64>,
    ratio_w: f64,
    ratio_h: f64,
    ratio_net: f64,
) -> Array3<f64> {
    let scale = array![ratio_w * ratio_net, ratio_h * ratio_net];
    polys * &scale
}

#[cfg(test)]
mod tests {
    use crate::{DbNetDetector, DefaultOptions};
    use interface::{
        detectors::{Detector, PreprocessorOptions},
        image::{CpuImageProcessor, ImageOp, RawImage},
        model::{CreateData, Model as _},
    };

    #[test]
    fn load_unload() {
        let mut data = DbNetDetector::new(CreateData::all(), false);
        data.load().expect("failed to load model");
        data.unload();
    }

    #[test]
    fn run() {
        let mut data = DbNetDetector::new(CreateData::all(), false);
        let cpu_image_processor =
            Box::new(CpuImageProcessor::default()) as Box<dyn ImageOp + Send + Sync>;
        data.load().expect("Failed to load data");
        data.detect(
            &RawImage::new("./imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.png")
                .expect("Failed to load image"),
            PreprocessorOptions::default(),
            DefaultOptions::default().dump(),
            &cpu_image_processor,
        )
        .expect("failed to detect");
    }
}
