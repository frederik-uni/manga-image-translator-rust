use std::collections::HashMap;

use maplit::hashmap;

use crate::detectors::{Detector, Model};

pub struct CraftDetector;

impl Detector for CraftDetector {
    fn detect(
        &self,
        detect: crate::detectors::TranslateTask,
        img_processor: Box<dyn crate::image::ImageOp>,
    ) -> bool {
        todo!()
    }

    fn models(&self) -> HashMap<&'static str, Model> {
        hashmap! {
            "refiner" => Model {
                url: "https://github.com/zyddnys/manga-image-translator/releases/download/beta-0.3/craft_refiner_CTW1500.pth",
                hash: "f7000cd3e9c76f2231b62b32182212203f73c08dfaa12bb16ffb529948a01399",
                file: Some("craft_refiner_CTW1500.pth"),
                archive: None,
            },
            "craft" => Model {
                url: "https://github.com/zyddnys/manga-image-translator/releases/download/beta-0.3/craft_mlt_25k.pth",
                hash: "4a5efbfb48b4081100544e75e1e2b57f8de3d84f213004b14b85fd4b3748db17",
                file: Some("craft_mlt_25k.pth"),
                archive: None,
            }
        }
    }
}
