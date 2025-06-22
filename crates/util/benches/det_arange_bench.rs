use criterion::{Criterion, criterion_group, criterion_main};
use interface::image::{CpuImageProcessor, ImageOp, RawImage, dummy::DummyImageProcessor};
use ndarray::Array4;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use util::det_arrange::det_rearrange_forward;

// Static shared db and mask
static DB: Lazy<Mutex<Option<Array4<f32>>>> = Lazy::new(|| Mutex::new(None));
static MASK: Lazy<Mutex<Option<Array4<f32>>>> = Lazy::new(|| Mutex::new(None));

fn mocking(_: Array4<u8>) -> (Array4<f32>, Array4<f32>) {
    let db = DB.lock().unwrap();
    let mask = MASK.lock().unwrap();
    (db.as_ref().unwrap().clone(), mask.as_ref().unwrap().clone())
}

fn bench_find_contours_from_ndarray(c: &mut Criterion) {
    let img = RawImage::new("./imgs/01_1-optimized.png").unwrap();
    let cpu = Box::new(CpuImageProcessor::default()) as Box<dyn ImageOp + Send + Sync>;

    {
        *DB.lock().unwrap() = Some(ndarray_npy::read_npy("npys/db.npy").unwrap());
        *MASK.lock().unwrap() = Some(ndarray_npy::read_npy("npys/mask.npy").unwrap());
    }

    c.bench_function("det_rearrange_forward", |b| {
        b.iter(|| {
            det_rearrange_forward(img.clone(), 2048, 4, mocking, &cpu).unwrap();
        });
    });
}

criterion_group!(benches, bench_find_contours_from_ndarray);
criterion_main!(benches);
