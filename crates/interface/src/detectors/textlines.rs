use geo::{Area as _, ConvexHull as _, MultiPoint, Point, Polygon};

#[derive(Debug)]
pub struct Quadrilateral {
    pts: [(i64, i64); 4],
    score: f64,
    vertical: bool,
}

impl Quadrilateral {
    pub fn pts(&self) -> &[(i64, i64)] {
        &self.pts
    }
    pub fn score(&self) -> f64 {
        self.score
    }

    pub fn vertical(&self) -> bool {
        self.vertical
    }

    pub fn new(points: Vec<(i64, i64)>, score: f64) -> Self {
        let pts: [(i64, i64); 4] = points.try_into().expect("Vec must have exactly 4 elements");

        let (pts, vertical) = sort_pnts(pts);
        Self {
            pts,
            score,
            vertical,
        }
    }

    pub fn polygon(&self) -> Polygon {
        let points: Vec<Point> = self
            .pts
            .iter()
            .map(|&p| Point::new(p.0 as f64, p.1 as f64))
            .collect();
        MultiPoint::from(points).convex_hull()
    }

    pub fn area(&self) -> f64 {
        self.polygon().unsigned_area()
    }

    pub fn structure(&self) -> [(i64, i64); 4] {
        let midpoint = |a: (i64, i64), b: (i64, i64)| ((a.0 + b.0) / 2, (a.1 + b.1) / 2);

        let p1 = midpoint(self.pts[0], self.pts[1]);
        let p2 = midpoint(self.pts[2], self.pts[3]);
        let p3 = midpoint(self.pts[1], self.pts[2]);
        let p4 = midpoint(self.pts[3], self.pts[0]);

        [p1, p2, p3, p4]
    }

    pub fn aspect_ratio(&self) -> f64 {
        let [l1a, l1b, l2a, l2b] = self.structure();

        let v1 = ((l1b.0 - l1a.0) as f64, (l1b.1 - l1a.1) as f64); // vertical
        let v2 = ((l2b.0 - l2a.0) as f64, (l2b.1 - l2a.1) as f64); // horizontal

        let norm = |v: (f64, f64)| (v.0.powi(2) + v.1.powi(2)).sqrt();

        let vertical_len = norm(v1);
        let horizontal_len = norm(v2);

        horizontal_len / vertical_len
    }
}

/// Direction must be provided for sorting.
/// The longer structure vector (mean of long side vectors) of input points is used to determine the direction.
/// It is reliable enough for text lines but not for blocks.
fn sort_pnts(pts: [(i64, i64); 4]) -> ([(i64, i64); 4], bool) {
    let mut pairwise_vec = [(0i64, 0i64); 16];
    let mut idx = 0;
    for i in 0..4 {
        for j in 0..4 {
            pairwise_vec[idx] = (pts[i].0 - pts[j].0, pts[i].1 - pts[j].1);
            idx += 1;
        }
    }

    let mut pairwise_vec_norm: [f64; 16] = [0.0; 16];
    for i in 0..16 {
        let (x, y) = pairwise_vec[i];
        pairwise_vec_norm[i] = ((x * x + y * y) as f64).sqrt();
    }

    let mut indices: [usize; 16] = {
        let mut tmp = [0; 16];
        for i in 0..16 {
            tmp[i] = i;
        }
        tmp
    };
    indices.sort_by(|&i, &j| {
        pairwise_vec_norm[i]
            .partial_cmp(&pairwise_vec_norm[j])
            .unwrap()
    });

    let long_side_ids = [indices[8], indices[10]];
    let mut long_side_vecs = [
        pairwise_vec[long_side_ids[0]],
        pairwise_vec[long_side_ids[1]],
    ];

    let inner_prod =
        long_side_vecs[0].0 * long_side_vecs[1].0 + long_side_vecs[0].1 * long_side_vecs[1].1;
    if inner_prod < 0 {
        long_side_vecs[0] = (-long_side_vecs[0].0, -long_side_vecs[0].1);
    }

    let sum_x = long_side_vecs[0].0 + long_side_vecs[1].0;
    let sum_y = long_side_vecs[0].1 + long_side_vecs[1].1;

    let mean_x = (sum_x as f64 / 2.0).abs();
    let mean_y = (sum_y as f64 / 2.0).abs();

    let is_vertical = mean_x <= mean_y;

    let mut pts_sorted = pts;

    if is_vertical {
        pts_sorted.sort_by_key(|&(_, y)| y);

        let mut top = [pts_sorted[0], pts_sorted[1]];
        let mut bottom = [pts_sorted[2], pts_sorted[3]];

        top.sort_by_key(|&(x, _)| x); // left to right
        bottom.sort_by_key(|&(x, _)| -x); // right to left

        ([top[0], top[1], bottom[0], bottom[1]], is_vertical)
    } else {
        pts_sorted.sort_by_key(|&(x, _)| x);

        let mut left = [pts_sorted[0], pts_sorted[1]];
        let mut right = [pts_sorted[2], pts_sorted[3]];

        left.sort_by_key(|&(_, y)| y); // top to bottom
        right.sort_by_key(|&(_, y)| y); // top to bottom

        ([left[0], right[0], right[1], left[1]], is_vertical)
    }
}

#[cfg(test)]
mod tests {
    use crate::detectors::textlines::sort_pnts;

    #[test]
    fn test_sort_pnts2() {
        let pts = [(0, 0), (10, 0), (0, 1), (10, 1)];
        // This forms a horizontal rectangle
        // Width is 10, height is 1, so long sides are horizontal
        let expected = [(0, 0), (10, 0), (10, 1), (0, 1)];
        let actual = sort_pnts(pts);

        assert_eq!(actual, (expected, false))
    }

    #[test]
    fn test_sort_pnts() {
        let pts = [(169, 6), (207, 6), (169, 164), (207, 164)];
        let expected = [(169, 6), (207, 6), (207, 164), (169, 164)];
        let actual = sort_pnts(pts);
        assert_eq!(actual, (expected, true))
    }

    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_quadrilateral_new_and_accessors() {
        let points = vec![(0, 0), (10, 0), (10, 1), (0, 1)];
        let quad = Quadrilateral::new(points.clone(), 0.9);

        assert_eq!(quad.pts(), &[(0, 0), (10, 0), (10, 1), (0, 1)]);
        assert_eq!(quad.score(), 0.9);
        assert!(!quad.vertical());
    }

    #[test]
    fn test_area() {
        let points = vec![(0, 0), (4, 0), (4, 3), (0, 3)];
        let quad = Quadrilateral::new(points, 1.0);

        let area = quad.area();
        assert_relative_eq!(area, 12.0, epsilon = 1e-6);
    }

    #[test]
    fn test_aspect_ratio_horizontal() {
        let points = vec![(0, 0), (10, 0), (10, 2), (0, 2)];
        let quad = Quadrilateral::new(points, 1.0);

        let aspect = quad.aspect_ratio();
        assert!(aspect > 4.5 && aspect < 5.5); // 10 / 2
    }

    #[test]
    fn test_aspect_ratio_vertical() {
        let points = vec![(0, 0), (2, 0), (2, 10), (0, 10)];
        let quad = Quadrilateral::new(points, 1.0);

        let aspect = quad.aspect_ratio();
        assert!(aspect < 0.3);
    }

    #[test]
    fn test_structure_midpoints() {
        let points = vec![(0, 0), (10, 0), (10, 4), (0, 4)];
        let quad = Quadrilateral::new(points, 1.0);
        let structure = quad.structure();

        assert_eq!(structure[0], (5, 0));
        assert_eq!(structure[1], (5, 4));
        assert_eq!(structure[2], (10, 2));
        assert_eq!(structure[3], (0, 2));
    }
}
