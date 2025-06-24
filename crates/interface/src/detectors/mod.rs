mod common;
pub mod modules;
pub mod textlines;

use base_util::RawSerializable;

use crate::{
    detectors::textlines::Quadrilateral,
    image::{DimType, ImageOp, RawImage},
    model::Model,
};

#[derive(Default, Clone, Copy)]
pub struct PreprocessorOptions {
    /// Invert the image colors for detection. Might improve detection.
    pub invert: bool,
    /// Applies gamma correction for detection. Might improve detection.
    pub gamma_correct: bool,
    /// Rotate the image for detection. Might improve detection.
    pub rotate: bool,
    /// Rotate the image for detection to prefer vertical textlines. Might improve detection.
    pub auto_rotate: bool,
}

impl PreprocessorOptions {
    pub fn set_auto_rotate(mut self, auto_rotate: bool) -> Self {
        self.auto_rotate = auto_rotate;
        self
    }
}

pub struct Data {}

// pub fn default_detect(
//     detector: &mut dyn Detector,
//     image: &RawImage,
//     pre_options: PreprocessorOptions,
//     options: &dyn Any,
//     img_processor: &Box<dyn ImageOp + Send + Sync>,
// ) -> anyhow::Result<(Vec<Quadrilateral>, Mask)> {

// }
//
//
fn test(item: Box<dyn Detector>) {}

pub trait Detector: Model {
    fn detect(
        &mut self,
        image: &RawImage,
        pre_processor_options: PreprocessorOptions,
        options: &[u8],
        img_processor: &Box<dyn ImageOp + Send + Sync>,
    ) -> anyhow::Result<(Vec<Quadrilateral>, Mask)> {
        let v = common::detect(image, &pre_processor_options, img_processor, |img| {
            self.infer(img, options, img_processor)
        })?;

        match v {
            Some(v) => Ok(v),
            None => self.detect(
                image,
                pre_processor_options.set_auto_rotate(false),
                options,
                img_processor,
            ),
        }
    }
    fn infer(
        &mut self,
        img: RawImage,
        options: &[u8],
        img_processor: &Box<dyn ImageOp + Send + Sync>,
    ) -> anyhow::Result<(Vec<Quadrilateral>, Mask)>;
}

#[derive(Clone)]
pub struct Mask {
    pub width: DimType,
    pub height: DimType,
    pub data: Vec<u8>,
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

impl RawSerializable for DefaultOptions {}
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
