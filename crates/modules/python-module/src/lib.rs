use std::collections::HashMap;

use interface::{detectors::Detector, model::Model};

pub struct PythonDetector {}

impl Model for PythonDetector {
    fn name(&self) -> &'static str {
        "python-module"
    }

    fn kind(&self) -> &'static str {
        "detector"
    }

    fn models(&self) -> std::collections::HashMap<&'static str, interface::model::ModelSource> {
        HashMap::new()
    }

    fn loaded(&self) -> bool {
        todo!()
    }

    fn unload(&mut self) {
        todo!()
    }

    fn load(&mut self) -> anyhow::Result<()> {
        todo!()
    }
}

impl Detector for PythonDetector {
    fn infer(
        &mut self,
        img: interface::image::RawImage,
        options: &[u8],
        img_processor: &Box<dyn interface::image::ImageOp + Send + Sync>,
    ) -> anyhow::Result<(
        Vec<interface::detectors::textlines::Quadrilateral>,
        interface::detectors::Mask,
    )> {
        todo!()
    }
}
