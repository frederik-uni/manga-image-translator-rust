use criterion::{Criterion, criterion_group, criterion_main};
use interface::{
    detectors::{Detector, PreprocessorOptions},
    image::{CpuImageProcessor, ImageOp, RawImage},
    model::{CreateData, Model as _},
};
use none::NoneDetector;

fn criterion_benchmark(c: &mut Criterion) {
    let mut data = NoneDetector::new(CreateData::all());
    let img = RawImage::new("./test.png").expect("Failed to load image");
    let cpu_image_processor =
        Box::new(CpuImageProcessor::default()) as Box<dyn ImageOp + Send + Sync>;

    c.bench_function("load_unload", |b| {
        b.iter(|| {
            data.load().expect("Failed to load model");
            data.unload();
        })
    });

    c.bench_function("infer", |b| {
        data.load().expect("Failed to load model");
        b.iter(|| data.infer(img.clone(), &[], &cpu_image_processor))
    });

    c.bench_function("detection", |b| {
        data.load().expect("Failed to load model");
        b.iter(|| {
            data.detect(
                &img,
                PreprocessorOptions::default(),
                &[],
                &cpu_image_processor,
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
