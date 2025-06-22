use criterion::{Criterion, criterion_group, criterion_main};
use interface::image::{Interpolation, RawImage, dummy::DummyImageProcessor};
use ndarray::Array2;
use std::hint::black_box;
use util::imageproc::{find_contours_from_ndarray, resize_aspect_ratio};
fn generate_test_bitmap(size: usize) -> Array2<bool> {
    Array2::from_shape_fn((size, size), |(i, j)| i == j || j == (size - i - 1))
}

fn bench_find_contours_from_ndarray(c: &mut Criterion) {
    let bitmap = generate_test_bitmap(512);

    let img = RawImage {
        width: 640,
        height: 480,
        channels: 3,
        data: vec![128; 640 * 480 * 3],
    };
    let op: Box<dyn interface::image::ImageOp + Send + Sync> = Box::new(DummyImageProcessor);

    c.bench_function("resize_aspect_ratio", |b| {
        b.iter(|| {
            resize_aspect_ratio(
                black_box(img.clone()),
                black_box(512),
                black_box(Interpolation::Bilinear),
                black_box(1.5),
                &op,
            )
        });
    });

    c.bench_function("find_contours_from_ndarray_512x512", |b| {
        b.iter(|| {
            let result = find_contours_from_ndarray(&black_box(bitmap.view()));
            assert!(result.is_ok());
        });
    });

    let large_bitmap = generate_test_bitmap(1024);

    c.bench_function("find_contours_from_ndarray_1024x1024", |b| {
        b.iter(|| {
            let result = find_contours_from_ndarray(&black_box(large_bitmap.view()));
            assert!(result.is_ok());
        });
    });
}

criterion_group!(benches, bench_find_contours_from_ndarray);
criterion_main!(benches);
