use criterion::{criterion_group, criterion_main, Criterion};
use interface::image::{CpuImageProcessor, ImageOp as _, RawImage, RayonImageProcessor};

fn bench_invert_cpu(processor: &mut CpuImageProcessor, image: &RawImage) -> RawImage {
    processor.histogram_equalization(image.clone())
}

fn bench_invert_rayon(processor: &mut RayonImageProcessor, image: &RawImage) -> RawImage {
    processor.histogram_equalization(image.clone())
}

#[cfg(feature = "gpu")]
fn bench_invert_gpu(processor: &mut crate::image::GpuImageProcessor, image: &RawImage) -> RawImage {
    processor.histogram_equalization(image.clone())
}

fn criterion_benchmark(c: &mut Criterion) {
    let image = RawImage::new("imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.png")
        .expect("Failed to load image");

    let mut cpu_processor = CpuImageProcessor::default();
    let mut rayon_processor = RayonImageProcessor::default();
    #[cfg(feature = "gpu")]
    let mut gpu_processor = crate::GpuImageProcessor::new();

    c.bench_function("histogram_equalization_cpu", |b| {
        b.iter(|| bench_invert_cpu(&mut cpu_processor, &image))
    });
    c.bench_function("histogram_equalization_rayon", |b| {
        b.iter(|| bench_invert_rayon(&mut rayon_processor, &image))
    });
    #[cfg(feature = "gpu")]
    c.bench_function("histogram_equalization_gpu", |b| {
        b.iter(|| bench_invert_gpu(&mut gpu_processor, &image))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
