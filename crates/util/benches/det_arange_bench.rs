use base_util::error::ProcessingError;
use criterion::{criterion_group, criterion_main, Criterion};
use interface::image::{dummy::DummyImageProcessor, CpuImageProcessor, ImageOp, RawImage};
use ndarray::Array4;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use util::det_arrange::det_rearrange_forward;

// Static shared db and mask
static DB: Lazy<Mutex<Option<Array4<f32>>>> = Lazy::new(|| Mutex::new(None));
static MASK: Lazy<Mutex<Option<Array4<f32>>>> = Lazy::new(|| Mutex::new(None));

fn mocking(_: Array4<u8>) -> Result<(Array4<f32>, Array4<f32>), ProcessingError> {
    let db = DB.lock().expect("mutex error");
    let mask = MASK.lock().expect("mutex error");
    Ok((
        db.as_ref().expect("db file not loaded yet").clone(),
        mask.as_ref().expect("mask file not loaded yet").clone(),
    ))
}

fn bench_find_contours_from_ndarray(c: &mut Criterion) {
    let img = RawImage::new("./imgs/01_1-optimized.png").expect("couldnt load image");
    let cpu = Box::new(CpuImageProcessor::default()) as Box<dyn ImageOp + Send + Sync>;

    {
        *DB.lock().expect("failed to lock DB") =
            Some(ndarray_npy::read_npy("npys/db.npy").expect("couldnt load npy"));
        *MASK.lock().expect("failed to lock MASK") =
            Some(ndarray_npy::read_npy("npys/mask.npy").expect("couldnt load npy"));
    }

    c.bench_function("det_rearrange_forward", |b| {
        b.iter(|| {
            det_rearrange_forward(img.clone(), 2048, 4, mocking, &cpu);
        });
    });
}

criterion_group!(benches, bench_find_contours_from_ndarray);
criterion_main!(benches);
