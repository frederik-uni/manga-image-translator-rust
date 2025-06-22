use std::collections::HashMap;

use crate::detectors::Detector;

struct NoneDetector {
    loaded: bool,
}

impl NoneDetector {
    fn new() -> Self {
        Self { loaded: true }
    }
}

impl Detector for NoneDetector {
    fn detect(
        &self,
        detect: crate::detectors::TranslateTask,
        img_processor: Box<dyn crate::image::ImageOp>,
    ) -> bool {
        todo!()
    }

    fn models(&self) -> std::collections::HashMap<&'static str, crate::detectors::Model> {
        HashMap::new()
    }

    fn loaded(&self) -> bool {
        true
    }

    fn unload(&mut self) -> anyhow::Result<()> {
        self.loaded = false;
        Ok(())
    }

    fn load(&mut self) -> anyhow::Result<()> {
        self.loaded = true;
        Ok(())
    }

    fn new(_: crate::detectors::CreateData) -> Box<Self> {
        todo!()
    }
}
