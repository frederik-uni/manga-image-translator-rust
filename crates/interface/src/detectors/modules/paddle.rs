use maplit::hashmap;

use crate::detectors::{Detector, Model};

pub struct PaddleDetector {}

impl Detector for PaddleDetector {
    fn detect(
        &self,
        detect: crate::detectors::TranslateTask,
        img_processor: Box<dyn crate::image::ImageOp>,
    ) -> bool {
        todo!()
    }

    fn models(&self) -> std::collections::HashMap<&'static str, crate::detectors::Model> {
        hashmap! {
            "det" => Model{
                url: "https://paddleocr.bj.bcebos.com/PP-OCRv4/chinese/ch_PP-OCRv4_det_server_infer.tar",
                hash: "0c0e4fc2ef31dcfbb45fb8d29bd8e702ec55a240d62c32ff814270d8be6e6179",
                file: None,
                archive: Some(hashmap!{
                    "ch_PP-OCRv4_det_server_infer/inference.pdiparams" => "ch_PP-OCRv4_det_server_infer/",
                    "ch_PP-OCRv4_det_server_infer/inference.pdiparams.info"=> "ch_PP-OCRv4_det_server_infer/",
                    "ch_PP-OCRv4_det_server_infer/inference.pdmodel"=>"ch_PP-OCRv4_det_server_infer/",
                }),
            },
            "rec"=> Model{
                url: "https://paddleocr.bj.bcebos.com/PP-OCRv4/chinese/ch_PP-OCRv4_rec_infer.tar",
                hash: "830ea228e20c2b30c4db9666066c48512f67a63f5b1a32d0d33dc9170040ce7d",
                file: None,

                archive: Some(hashmap!{
                    "ch_PP-OCRv4_rec_infer/inference.pdiparams"=> "ch_PP-OCRv4_rec_infer/",
                    "ch_PP-OCRv4_rec_infer/inference.pdiparams.info"=> "ch_PP-OCRv4_rec_infer/",
                    "ch_PP-OCRv4_rec_infer/inference.pdmodel"=> "ch_PP-OCRv4_rec_infer/",
                }),
            },
            "cls"=> Model {
                url: "https://paddleocr.bj.bcebos.com/dygraph_v2.0/ch/ch_ppocr_mobile_v2.0_cls_infer.tar",
                hash: "507352585040d035da3b1e6374694ad679a850acb0a36a8d0d47984176357717",
                file: None,
                archive: Some(hashmap!{
                    "ch_ppocr_mobile_v2.0_cls_infer/inference.pdiparams"=> "ch_ppocr_mobile_v2.0_cls_infer/",
                    "ch_ppocr_mobile_v2.0_cls_infer/inference.pdmodel"=> "ch_ppocr_mobile_v2.0_cls_infer/",
                }),
            },
        }
    }
}
