use criterion::{criterion_group, criterion_main, Criterion};
use paddle::fill_polys_mask;
use rand::Rng as _;

pub fn generate_random_non_overlapping_rects(
    count: usize,
    width: usize,
    height: usize,
    max_coverage: f64,
) -> Vec<[(i64, i64); 4]> {
    let mut rng = rand::thread_rng();
    let mut rects: Vec<[(i64, i64); 4]> = Vec::with_capacity(count);

    let image_area = (width * height) as f64;
    let target_area = image_area * max_coverage;

    let mut total_area = 0.0;

    'outer: while rects.len() < count && total_area < target_area {
        let w = rng.gen_range((width as f64 * 0.05) as i64..=(width as f64 * 0.2) as i64);
        let h = rng.gen_range((height as f64 * 0.05) as i64..=(height as f64 * 0.2) as i64);

        let x0 = rng.gen_range(0..=(width as i64 - w));
        let y0 = rng.gen_range(0..=(height as i64 - h));
        let x1 = x0 + w;
        let y1 = y0 + h;

        for &[(ax0, ay0), (_, ay1), (ax1, _), _] in &rects {
            let overlap_x = (x0 < ax1) && (x1 > ax0);
            let overlap_y = (y0 < ay1) && (y1 > ay0);
            if overlap_x && overlap_y {
                continue 'outer;
            }
        }

        rects.push([(x0, y0), (x0, y1), (x1, y1), (x1, y0)]);
        total_area += (w * h) as f64;
    }

    rects
}

fn bench_fill_polys(c: &mut Criterion) {
    let width = 10000;
    let height = 10000;
    let count = 20;
    let max_coverage = 0.3;

    let pts = generate_random_non_overlapping_rects(count, width, height, max_coverage);
    let pts = pts.iter().collect::<Vec<_>>();

    c.bench_function("fill_polys_mask", |b| {
        b.iter(|| fill_polys_mask(pts.clone(), width, height))
    });
}

criterion_group!(benches, bench_fill_polys);
criterion_main!(benches);
