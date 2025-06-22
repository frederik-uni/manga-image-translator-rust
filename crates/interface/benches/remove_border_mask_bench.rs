use criterion::{Criterion, criterion_group, criterion_main};
use interface::{
    detectors::Mask,
    image::{CpuImageProcessor, ImageOp as _, RayonImageProcessor},
};

fn bench_remove_border_mask_cpu(processor: &mut CpuImageProcessor, image: &Mask) -> Mask {
    processor.remove_border_mask(image.clone(), 2000, 2000)
}

fn bench_remove_border_mask_rayon(processor: &mut RayonImageProcessor, image: &Mask) -> Mask {
    processor.remove_border_mask(image.clone(), 2000, 2000)
}

#[cfg(feature = "gpu")]
fn bench_remove_border_mask_gpu(
    processor: &mut crate::image::GpuImageProcessor,
    image: &Mask,
) -> Mask {
    processor.remove_border_mask(image.clone(), 2000, 2000)
}

fn criterion_benchmark(c: &mut Criterion) {
    let image = Mask {
        width: 3000,
        height: 2000,
        data: vec![0; 3000 * 2000],
    };

    let mut cpu_processor = CpuImageProcessor::default();
    let mut rayon_processor = RayonImageProcessor::default();
    #[cfg(feature = "gpu")]
    let mut gpu_processor = crate::image::GpuImageProcessor::new();

    c.bench_function("remove_border_mask_cpu", |b| {
        b.iter(|| bench_remove_border_mask_cpu(&mut cpu_processor, &image))
    });
    c.bench_function("remove_border_mask_rayon", |b| {
        b.iter(|| bench_remove_border_mask_rayon(&mut rayon_processor, &image))
    });
    #[cfg(feature = "gpu")]
    c.bench_function("remove_border_mask_gpu", |b| {
        b.iter(|| bench_remove_border_mask_gpu(&mut gpu_processor, &image))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
