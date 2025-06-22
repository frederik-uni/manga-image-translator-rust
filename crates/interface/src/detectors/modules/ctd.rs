use std::collections::HashMap;

use maplit::hashmap;

use crate::detectors::{Detector, Model};

pub struct CtdDetector {}

impl Detector for CtdDetector {
    fn detect(
        &self,
        detect: crate::detectors::TranslateTask,
        img_processor: Box<dyn crate::image::ImageOp>,
    ) -> bool {
        todo!()
    }

    fn models(&self) -> HashMap<&'static str, Model> {
        hashmap! {
            "model-cuda" => Model {
                url: "https://github.com/zyddnys/manga-image-translator/releases/download/beta-0.3/comictextdetector.pt",
                hash: "1f90fa60aeeb1eb82e2ac1167a66bf139a8a61b8780acd351ead55268540cccb",
                file: Some("."),
                archive: None,
            },
            "model-cpu" => Model {url:"https://github.com/zyddnys/manga-image-translator/releases/download/beta-0.3/comictextdetector.pt.onnx",hash:"1a86ace74961413cbd650002e7bb4dcec4980ffa21b2f19b86933372071d718f",file:Some("."), archive: None },
        }
    }
}
