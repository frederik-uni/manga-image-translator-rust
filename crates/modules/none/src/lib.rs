use std::{collections::HashMap, vec};

use interface::{
    detectors::{Detector, Mask},
    model::Model,
};

pub struct NoneDetector {
    loaded: bool,
}

impl NoneDetector {
    pub fn new(_: interface::model::CreateData) -> Box<Self> {
        Box::new(NoneDetector { loaded: true })
    }
}

impl Model for NoneDetector {
    fn name(&self) -> &'static str {
        "none"
    }

    fn kind(&self) -> &'static str {
        "detector"
    }

    fn models(&self) -> std::collections::HashMap<&'static str, interface::model::ModelSource> {
        HashMap::new()
    }

    fn loaded(&self) -> bool {
        self.loaded
    }

    fn unload(&mut self) {
        self.loaded = false;
    }

    fn load(&mut self) -> anyhow::Result<()> {
        self.loaded = true;
        Ok(())
    }
}

impl Detector for NoneDetector {
    fn infer(
        &mut self,
        img: interface::image::RawImage,
        _: &[u8],
        _: &Box<dyn interface::image::ImageOp + Send + Sync>,
    ) -> anyhow::Result<(
        Vec<interface::detectors::textlines::Quadrilateral>,
        interface::detectors::Mask,
    )> {
        Ok((
            vec![],
            Mask {
                width: img.width,
                height: img.height,
                data: Vec::new(),
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use interface::{
        detectors::{Detector, PreprocessorOptions},
        image::{CpuImageProcessor, ImageOp, RawImage},
        model::{CreateData, Model as _},
    };

    use crate::NoneDetector;

    #[test]
    fn load_unload() {
        let mut data = NoneDetector::new(CreateData::all());
        data.load().expect("failed to load model");
        data.unload();
    }

    #[test]
    fn run() {
        let mut data = NoneDetector::new(CreateData::all());
        let cpu_image_processor =
            Box::new(CpuImageProcessor::default()) as Box<dyn ImageOp + Send + Sync>;
        data.load().expect("Failed to load data");
        data.detect(
            &RawImage::new("./test.png").expect("Failed to load image"),
            PreprocessorOptions::default(),
            &[],
            &cpu_image_processor,
        )
        .expect("failed to detect");
    }
}
