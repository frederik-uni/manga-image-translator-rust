use criterion::{criterion_group, criterion_main, Criterion};
use interface::image::{
    CpuImageProcessor, ImageOp as _, Interpolation, RawImage, RayonImageProcessor,
};

fn bench_resize_cpu(
    processor: &mut CpuImageProcessor,
    image: &RawImage,
    interpolation: Interpolation,
) -> RawImage {
    processor.resize(
        image.clone(),
        image.width * 2,
        image.height * 2,
        interpolation,
    )
}

fn bench_resize_rayon(
    processor: &mut RayonImageProcessor,
    image: &RawImage,
    interpolation: Interpolation,
) -> RawImage {
    processor.resize(
        image.clone(),
        image.width * 2,
        image.height * 2,
        interpolation,
    )
}

#[cfg(feature = "gpu")]
fn bench_resize_gpu(
    processor: &mut crate::image::GpuImageProcessor,
    image: &RawImage,
    interpolation: Interpolation,
) -> RawImage {
    processor.resize(
        image.clone(),
        image.width * 2,
        image.height * 2,
        interpolation,
    )
}

fn criterion_benchmark(c: &mut Criterion) {
    let image = RawImage::new("imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.png")
        .expect("Failed to load image");

    let mut cpu_processor = CpuImageProcessor::default();
    let mut rayon_processor = RayonImageProcessor::default();
    #[cfg(feature = "gpu")]
    let mut gpu_processor = crate::image::GpuImageProcessor::new();

    c.bench_function("resize_bicubic_cpu", |b| {
        b.iter(|| bench_resize_cpu(&mut cpu_processor, &image, Interpolation::Bicubic))
    });
    c.bench_function("resize_bicubic_rayon", |b| {
        b.iter(|| bench_resize_rayon(&mut rayon_processor, &image, Interpolation::Bicubic))
    });
    #[cfg(feature = "gpu")]
    c.bench_function("resize_bicubic_gpu", |b| {
        b.iter(|| bench_resize_gpu(&mut gpu_processor, &image, Interpolation::Bicubic))
    });

    c.bench_function("resize_bilinear_cpu", |b| {
        b.iter(|| bench_resize_cpu(&mut cpu_processor, &image, Interpolation::Bilinear))
    });
    c.bench_function("resize_bilinear_rayon", |b| {
        b.iter(|| bench_resize_rayon(&mut rayon_processor, &image, Interpolation::Bilinear))
    });
    #[cfg(feature = "gpu")]
    c.bench_function("resize_bilinear_gpu", |b| {
        b.iter(|| bench_resize_gpu(&mut gpu_processor, &image, Interpolation::Bilinear))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
