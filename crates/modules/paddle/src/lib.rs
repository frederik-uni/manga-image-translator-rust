use std::f32;

use base_util::{
    error::ModelLoadError,
    onnx::{all_providers, new_session_},
    project::root_path,
    RawSerializable,
};
use geo::{MinimumRotatedRect, Point};

use interface::{
    detectors::{textlines::Quadrilateral, DefaultOptions, Detector, Mask},
    model::{CreateData, Model, ModelSource},
};
use maplit::hashmap;
use ort::session::builder::SessionBuilder;
use paddle_ocr_rs::{base_net::BaseNet, db_net::DbNet, scale_param::ScaleParam};

pub struct PaddleDetector {
    db_net: Option<DbNet>,
    db: CreateData,
}

impl PaddleDetector {
    ///convnext: Different model architecture, but based on dbnet
    pub fn new(db: CreateData) -> Self {
        PaddleDetector { db, db_net: None }
    }
}

fn ses_builder(v: SessionBuilder) -> Result<SessionBuilder, ort::Error> {
    new_session_(v, all_providers())
}

impl Model for PaddleDetector {
    fn name(&self) -> &'static str {
        "paddle"
    }

    fn kind(&self) -> &'static str {
        "detector"
    }

    fn models(&self) -> std::collections::HashMap<&'static str, interface::model::ModelSource> {
        hashmap! {
            "det" => ModelSource { url: "https://github.com/frederik-uni/manga-image-translator-rust/releases/download/paddle-ocr-chinese-v4/det.onnx", hash: "b21a993484b367c0ea29d4a703c038d6ee3212173e6abf962b09188b032a9483" },
        }
    }

    fn loaded(&self) -> bool {
        self.db_net.is_some()
    }

    fn unload(&mut self) {
        self.db_net = None;
    }

    fn load(&mut self) -> Result<(), ModelLoadError> {
        let mut db_net = DbNet::new();

        let p = root_path().join("models/detector/paddle/det.onnx");
        let url = self.models();
        let url = url.get("det").unwrap();
        let model = self
            .db
            .mode_db
            .get(self.kind(), self.name(), "det.onnx", url.url, url.hash)
            .unwrap()
            .to_string_lossy()
            .to_string();
        let err = db_net.init_model(&model, 0, Some(|v| ses_builder(v)));
        if let Err(e) = err {
            Err(match e {
                paddle_ocr_rs::ocr_error::OcrError::Ort(error) => {
                    ModelLoadError::CreateSession(error)
                }
                _ => unreachable!(),
            })?;
        }
        self.db_net = Some(db_net);
        Ok(())
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PaddleOptions {
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

impl RawSerializable for PaddleOptions {}

impl Default for PaddleOptions {
    fn default() -> Self {
        Self {
            unclip_ratio: 2.3,
            text_threshold: 0.5,
            box_threshold: 0.7,
        }
    }
}

impl Detector for PaddleDetector {
    fn infer(
        &mut self,
        img: interface::image::RawImage,
        options: &[u8],
        _: &Box<dyn interface::image::ImageOp + Send + Sync>,
    ) -> anyhow::Result<(
        Vec<interface::detectors::textlines::Quadrilateral>,
        interface::detectors::Mask,
    )> {
        let options = DefaultOptions::parse(options)?;
        let db_net = match &mut self.db_net {
            None => {
                self.load()?;
                self.db_net.as_mut().expect("Model should be loaded")
            }
            Some(model) => model,
        };

        let max_side_len = 960;
        let origin_max_side = img.width.max(img.height);

        let resize = if max_side_len <= 0 || max_side_len > origin_max_side {
            origin_max_side
        } else {
            max_side_len
        };
        // resize += 2 * padding;

        // let padding_src = OcrUtils::make_padding(img_src, padding).unwrap();
        let (w, h) = (img.width, img.height);
        let img = img.to_image().unwrap();

        let scale = ScaleParam::get_scale_param(&img, resize as u32);

        let text_boxes = db_net
            .get_text_boxes(
                &img,
                &scale,
                options.text_threshold as f32,
                options.box_threshold as f32,
                options.unclip_ratio as f32,
            )
            .unwrap();
        let boxes = text_boxes
            .into_iter()
            .filter(|v| v.score != f32::INFINITY)
            .map(|v| {
                let pts = v
                    .points
                    .into_iter()
                    .map(|v| (v.x as i64, v.y as i64))
                    .collect::<Vec<_>>();
                let poly = Quadrilateral::new(pts, 0.0).polygon();
                let corners: Vec<Point> = poly
                    .minimum_rotated_rect()
                    .unwrap()
                    .exterior()
                    .points_iter()
                    .take(4)
                    .collect();
                let rolled: Vec<_> = corners
                    .into_iter()
                    .cycle()
                    .skip(2)
                    .take(4)
                    .map(|v| (v.0.x as i64, v.0.y as i64))
                    .collect();
                Quadrilateral::new(rolled, v.score as f64)
            })
            .collect::<Vec<_>>();

        let area = fill_polys_mask(
            boxes.iter().map(|v| v.pts()).collect(),
            w as usize,
            h as usize,
        );

        let mask = Mask {
            width: w,
            height: h,
            data: area,
        };

        Ok((boxes, mask))
    }
}

pub fn fill_polys_mask(pts: Vec<&[(i64, i64); 4]>, width: usize, height: usize) -> Vec<u8> {
    let mut mask = vec![0u8; width * height];

    for quad in pts {
        fill_polygon(&mut mask, width, height, quad);
    }

    mask
}
fn fill_polygon(mask: &mut [u8], width: usize, height: usize, poly: &[(i64, i64); 4]) {
    let mut edges = Vec::new();

    for i in 0..4 {
        let (x0, y0) = poly[i];
        let (x1, y1) = poly[(i + 1) % 4];
        if y0 != y1 {
            edges.push(((x0, y0), (x1, y1)));
        }
    }

    let min_y = poly.iter().map(|&(_, y)| y).min().unwrap().max(0) as usize;
    let max_y = poly
        .iter()
        .map(|&(_, y)| y)
        .max()
        .unwrap()
        .min(height as i64 - 1) as usize;

    for y in min_y..=max_y {
        let y = y as i64;
        let mut x_intersections = vec![];

        for &((x0, y0), (x1, y1)) in &edges {
            if (y0 <= y && y < y1) || (y1 <= y && y < y0) {
                let dy = y1 - y0;
                let dx = x1 - x0;
                let t = (y - y0) as f64 / dy as f64;
                let x = x0 as f64 + t * dx as f64;
                x_intersections.push(x as i64);
            }
        }

        x_intersections.sort_unstable();
        for pair in x_intersections.chunks(2) {
            if let [x_start, x_end] = *pair {
                let x0 = x_start.max(0).min(width as i64 - 1) as usize;
                let x1 = x_end.max(0).min(width as i64 - 1) as usize;
                for x in x0..=x1 {
                    mask[y as usize * width + x] = 255;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{PaddleDetector, PaddleOptions};
    use base_util::RawSerializable as _;
    use interface::{
        detectors::{Detector, PreprocessorOptions},
        image::{CpuImageProcessor, ImageOp, RawImage},
        model::{CreateData, Model as _},
    };

    #[test]
    fn load_unload() {
        let mut data = PaddleDetector::new(CreateData::all());
        data.load().expect("failed to load model");
        data.unload();
    }

    #[test]
    fn run() {
        let mut data = PaddleDetector::new(CreateData::all());
        let cpu_image_processor =
            Box::new(CpuImageProcessor::default()) as Box<dyn ImageOp + Send + Sync>;
        data.load().expect("Failed to load data");
        data.detect(
            &RawImage::new("./imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.png")
                .expect("Failed to load image"),
            PreprocessorOptions::default(),
            PaddleOptions::default().dump(),
            &cpu_image_processor,
        )
        .expect("failed to detect");
    }
}
