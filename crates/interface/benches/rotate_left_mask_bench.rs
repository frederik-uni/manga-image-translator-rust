use criterion::{criterion_group, criterion_main, Criterion};
#[cfg(feature = "gpu")]
use interface::detectors::Mask;
use interface::{
    detectors::Mask,
    image::{CpuImageProcessor, ImageOp as _, RayonImageProcessor},
};

fn bench_rotate_mask_cpu(processor: &mut CpuImageProcessor, image: &Mask) -> Mask {
    processor.rotate_left_mask(image.clone())
}

fn bench_rotate_mask_rayon(processor: &mut RayonImageProcessor, image: &Mask) -> Mask {
    processor.rotate_left_mask(image.clone())
}

#[cfg(feature = "gpu")]
fn bench_rotate_mask_gpu(processor: &mut crate::image::GpuImageProcessor, image: &Mask) -> Mask {
    processor.rotate_left_mask(image.clone())
}

fn criterion_benchmark(c: &mut Criterion) {
    let image = Mask {
        width: 2000,
        height: 3000,
        data: vec![0; 2000 * 3000],
    };

    let mut cpu_processor = CpuImageProcessor::default();
    let mut rayon_processor = RayonImageProcessor::default();
    #[cfg(feature = "gpu")]
    let mut gpu_processor = crate::image::GpuImageProcessor::new();

    c.bench_function("rotate_mask_cpu", |b| {
        b.iter(|| bench_rotate_mask_cpu(&mut cpu_processor, &image))
    });
    c.bench_function("rotate_mask_rayon", |b| {
        b.iter(|| bench_rotate_mask_rayon(&mut rayon_processor, &image))
    });
    #[cfg(feature = "gpu")]
    c.bench_function("rotate_mask_gpu", |b| {
        b.iter(|| bench_rotate_mask_gpu(&mut gpu_processor, &image))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
