mod common;
pub mod modules;
pub mod textlines;

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
