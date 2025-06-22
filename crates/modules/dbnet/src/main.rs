use dbnet::{DbNetDetector, DefaultOptions};
use interface::{
    detectors::{Detector, PreprocessorOptions},
    image::{CpuImageProcessor, ImageOp, RawImage},
    model::{CreateData, Model as _},
};

fn main() {
    env_logger::init();
    let mut data = DbNetDetector::new(CreateData::all(), false);
    let cpu_image_processor =
        Box::new(CpuImageProcessor::default()) as Box<dyn ImageOp + Send + Sync>;
    data.load().expect("Failed to load data");

    let (_, _) = data
        .detect(
            &RawImage::new("./test.png").expect("Failed to load image"),
            PreprocessorOptions::default(),
            DefaultOptions::default().dump(),
            &cpu_image_processor,
        )
        .expect("failed to detect");
}
