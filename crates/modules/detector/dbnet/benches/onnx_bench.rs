use base_util::RawSerializable;
use criterion::{criterion_group, criterion_main, Criterion};
use dbnet::DbNetDetector;
use interface::{
    detectors::{DefaultOptions, Detector, PreprocessorOptions},
    image::{CpuImageProcessor, ImageOp, RawImage},
    model::{CreateData, Model as _},
};

fn criterion_benchmark(c: &mut Criterion) {
    let mut data = DbNetDetector::new(CreateData::all(), false);
    let img = RawImage::new("./imgs/232264684-5a7bcf8e-707b-4925-86b0-4212382f1680.png")
        .expect("Failed to load image");
    let cpu_image_processor =
        Box::new(CpuImageProcessor::default()) as Box<dyn ImageOp + Send + Sync>;

    // c.bench_function("load_unload", |b| {
    //     b.iter(|| {
    //         data.load().expect("Failed to load model");
    //         data.unload();
    //     })
    // });

    c.bench_function("infer", |b| {
        data.load().expect("Failed to load model");
        b.iter(|| {
            data.infer(
                img.clone(),
                DefaultOptions::default().dump(),
                &cpu_image_processor,
            )
        })
    });

    c.bench_function("detection", |b| {
        data.load().expect("Failed to load model");
        b.iter(|| {
            data.detect(
                &img,
                PreprocessorOptions::default(),
                DefaultOptions::default().dump(),
                &cpu_image_processor,
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
