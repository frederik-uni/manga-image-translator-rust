use base_util::RawSerializable;
use dbnet::DbNetDetector;
use interface::{
    detectors::{DefaultOptions, Detector, PreprocessorOptions},
    image::{CpuImageProcessor, ImageOp, RawImage},
    model::{CreateData, Model as _},
};
fn main() {
    env_logger::init();
    let mut data = DbNetDetector::new(CreateData::all(), false);
    let cpu_image_processor =
        Box::new(CpuImageProcessor::default()) as Box<dyn ImageOp + Send + Sync>;
    data.load().expect("Failed to load data");
    let img = RawImage::new("./imgs/01_1-optimized.png").expect("Failed to load image");
    let (_, _) = data
        .detect(
            &img,
            PreprocessorOptions::default(),
            DefaultOptions::default().dump(),
            &cpu_image_processor,
        )
        .expect("failed to detect");
}
