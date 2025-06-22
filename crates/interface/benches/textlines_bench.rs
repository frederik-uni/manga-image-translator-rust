use criterion::{Criterion, criterion_group, criterion_main};
use interface::detectors::textlines::Quadrilateral;

fn bench_create(points: Vec<(i64, i64)>, score: f64) {
    Quadrilateral::new(points, score);
}

fn bench_area(quadrilateral: &Quadrilateral) {
    quadrilateral.area();
}

fn bench_aspect_ratio(quadrilateral: &Quadrilateral) {
    quadrilateral.aspect_ratio();
}

fn bench_structure(quadrilateral: &Quadrilateral) {
    quadrilateral.structure();
}

fn bench_polygon(quadrilateral: &Quadrilateral) {
    quadrilateral.polygon();
}

fn criterion_benchmark(c: &mut Criterion) {
    let pts1 = vec![(0, 0), (10, 0), (0, 1), (10, 1)];
    let pts2 = vec![(169, 6), (207, 6), (169, 164), (207, 164)];
    let line = Quadrilateral::new(pts2.clone(), 0.7);

    c.bench_function("create1", |b| b.iter(|| bench_create(pts1.clone(), 0.7)));
    c.bench_function("create2", |b| b.iter(|| bench_create(pts2.clone(), 0.7)));
    c.bench_function("area", |b| b.iter(|| bench_area(&line)));
    c.bench_function("aspect_ratio", |b| b.iter(|| bench_aspect_ratio(&line)));
    c.bench_function("structure", |b| b.iter(|| bench_structure(&line)));
    c.bench_function("polygon", |b| b.iter(|| bench_polygon(&line)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
