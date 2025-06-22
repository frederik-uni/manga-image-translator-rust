use clipper2::{ClipperOffset, ClipperOffsetConfig, Path};
use clipper2c_sys::{ClipperEndType_POLYGON_END, ClipperJoinType_ROUND_JOIN};
use ndarray::{Array1, Array2, Array3, Array4, ArrayView2, Axis, concatenate, s, stack};
use opencv::{
    core::{CV_8UC1, Mat, MatExprTraitConst as _, MatTraitConstManual, Point, Scalar, Vector},
    imgproc,
};

//TODO: refactor + test + bench

pub struct SegDetectorRepresenter {
    pub min_size: f32,
    pub thresh: f32,
    pub box_thresh: f64,
    pub max_candidates: usize,
    pub unclip_ratio: f64,
}

pub struct Batch {
    pub shape: Vec<(u16, u16)>,
}

#[derive(Copy, Clone)]
pub struct MyPoint {
    pub x: f32,
    pub y: f32,
}

impl MyPoint {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

fn roll_rows(arr: &Array2<f32>, shift: isize) -> Array2<f32> {
    let n_rows = arr.shape()[0] as isize;
    if n_rows == 0 {
        return arr.clone();
    }

    // Normalize shift to positive value
    let normalized_shift = (shift % n_rows + n_rows) % n_rows;
    let split_point = (n_rows - normalized_shift) as usize;

    // Split array into two parts
    let (top, bottom) = arr.view().split_at(Axis(0), split_point);

    // Concatenate parts in reverse order
    concatenate![Axis(0), bottom, top]
}
impl SegDetectorRepresenter {
    fn binarize(&self, pred: &Array3<f32>) -> Array3<bool> {
        pred.mapv(|x| x > self.thresh)
    }

    /// batch: (image, polygons, ignore_tags
    /// batch: a dict produced by dataloaders.
    ///     image: tensor of shape (N, C, H, W).
    ///     polygons: tensor of shape (N, K, 4, 2), the polygons of objective regions.
    ///     ignore_tags: tensor of shape (N, K), indicates whether a region is ignorable or not.
    ///     shape: the original shape of images.
    ///     filename: the original filenames of images.
    /// pred:
    ///     binary: text region segmentation map, with shape (N, H, W)
    ///     thresh: [if exists] thresh hold prediction with shape (N, H, W)
    ///     thresh_binary: [if exists] binarized with threshold, (N, H, W)
    pub fn call(
        &self,
        batch: Batch,
        pred: Array4<f32>,
        is_output_polygon: bool,
    ) -> (Vec<Option<Array3<i64>>>, Vec<Option<Vec<f64>>>) {
        let pred: Array3<f32> = pred.slice(s![.., 0, .., ..]).to_owned();
        let segmentation: Array3<bool> = self.binarize(&pred);
        let batch_size = pred.shape()[0];
        let (boxes_batch, scores_batch): (Vec<Option<Array3<i64>>>, Vec<Option<Vec<f64>>>) = batch
            .shape[0..batch_size]
            .iter()
            .enumerate()
            .map(|(batch_index, (height, width))| match is_output_polygon {
                true => self.polygons_from_bitmap(
                    pred.index_axis(Axis(0), batch_index),
                    segmentation.index_axis(Axis(0), batch_index),
                    *width,
                    *height,
                ),
                false => self.boxes_from_bitmap(
                    pred.index_axis(Axis(0), batch_index),
                    segmentation.index_axis(Axis(0), batch_index),
                    *width,
                    *height,
                ),
            })
            .unzip();

        (boxes_batch, scores_batch)
    }

    fn polygons_from_bitmap(
        &self,
        _: ArrayView2<f32>,
        _: ArrayView2<bool>,
        _: u16,
        _: u16,
    ) -> (Option<Array3<i64>>, Option<Vec<f64>>) {
        unimplemented!()
    }

    fn get_mini_boxes(contour: &Vector<Point>) -> (Vec<(f32, f32)>, f32) {
        let box_ = opencv::imgproc::min_area_rect(&contour).unwrap();
        let mut points: Mat = Mat::default();
        opencv::imgproc::box_points(box_, &mut points).unwrap();

        let points: Vec<Vec<f32>> = points.to_vec_2d().unwrap();
        let mut points_vec = points
            .into_iter()
            .map(|v| MyPoint::new(v[0], v[1]))
            .collect::<Vec<MyPoint>>();
        points_vec.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

        let (index_1, index_4) = if points_vec[1].y > points_vec[0].y {
            (0, 1)
        } else {
            (1, 0)
        };
        let (index_2, index_3) = if points_vec[3].y > points_vec[2].y {
            (2, 3)
        } else {
            (3, 2)
        };

        let box_points = vec![
            points_vec[index_1],
            points_vec[index_2],
            points_vec[index_3],
            points_vec[index_4],
        ]
        .into_iter()
        .map(|v| (v.x, v.y))
        .collect::<Vec<_>>();

        let min_side = box_.size.width.min(box_.size.height);

        (box_points, min_side)
    }

    pub fn box_score_fast(bitmap: ArrayView2<f32>, box_: &Vector<Point>) -> opencv::Result<f64> {
        let (h, w) = (bitmap.nrows() as i32, bitmap.ncols() as i32);

        let xs: Vec<i32> = box_.iter().map(|p| p.x).collect();
        let ys: Vec<i32> = box_.iter().map(|p| p.y).collect();

        let xmin = *xs.iter().min().unwrap().clamp(&0, &(w - 1));
        let xmax = *xs.iter().max().unwrap().clamp(&0, &(w - 1));
        let ymin = *ys.iter().min().unwrap().clamp(&0, &(h - 1));
        let ymax = *ys.iter().max().unwrap().clamp(&0, &(h - 1));

        let width = xmax - xmin + 1;
        let height = ymax - ymin + 1;

        let mut mask = Mat::zeros(height, width, CV_8UC1)?.to_mat()?;

        let mut adj_points = Vector::<Point>::new();
        for p in box_ {
            adj_points.push(Point::new(p.x - xmin, p.y - ymin));
        }
        let mut pts_vec = Vector::<Vector<Point>>::new();
        pts_vec.push(adj_points);

        imgproc::fill_poly(
            &mut mask,
            &pts_vec,
            Scalar::all(1.0),
            imgproc::LINE_8,
            0,
            Point::new(0, 0),
        )?;

        let roi = bitmap.slice(s![
            ymin as usize..=ymax as usize,
            xmin as usize..=xmax as usize
        ]);

        let mask_array = {
            let mask_bytes = mask.data_bytes()?;
            ArrayView2::from_shape((height as usize, width as usize), mask_bytes).unwrap()
        };

        let mut sum = 0.0f64;
        let mut count = 0usize;
        for (&pixel, &m) in roi.iter().zip(mask_array.iter()) {
            if m != 0 {
                sum += pixel as f64;
                count += 1;
            }
        }

        Ok(if count > 0 { sum / (count as f64) } else { 0.0 })
    }

    fn boxes_from_bitmap(
        &self,
        pred: ArrayView2<f32>,
        bitmap: ArrayView2<bool>,
        dest_width: u16,
        dest_height: u16,
    ) -> (Option<Array3<i64>>, Option<Vec<f64>>) {
        let [height, width] = bitmap.shape()[..] else {
            panic!("Expected 2 dimensions");
        };

        let contours = match crate::imageproc::find_contours_from_ndarray(&bitmap) {
            Ok(v) => v,
            Err(_) => return (None, None),
        };
        let num_contours = contours.len().min(self.max_candidates);
        let mut boxes: Array3<i64> = Array3::zeros((num_contours, 4, 2));

        let mut scores: Vec<f64> = vec![0.0; num_contours];
        for index in 0..num_contours {
            let contour = contours.get(index).unwrap();
            let (points, sside) = Self::get_mini_boxes(&contour);
            if sside < self.min_size {
                continue;
            }
            let score = Self::box_score_fast(pred, &contour).unwrap();
            if self.box_thresh > score {
                continue;
            }

            let box_ = Self::unclip(points, self.unclip_ratio)
                .into_iter()
                .flatten()
                .map(|(x, y)| Point::new(x as i32, y as i32));

            let (box_, sside) = Self::get_mini_boxes(&Vector::from_iter(box_));
            if sside < self.min_size + 2.0 {
                continue;
            }

            let (x, y): (Vec<_>, Vec<_>) = box_.into_iter().unzip();
            let x = x
                .into_iter()
                .map(|v| {
                    (v / width as f32 * dest_width as f32)
                        .round()
                        .clamp(0.0, dest_width as f32)
                })
                .collect::<Array1<_>>();
            let y = y
                .into_iter()
                .map(|v| {
                    (v / height as f32 * dest_height as f32)
                        .round()
                        .clamp(0.0, dest_height as f32)
                })
                .collect::<Array1<_>>();
            let box_ = stack![Axis(1), x, y];
            let startidx = box_
                .sum_axis(Axis(1))
                .iter()
                .enumerate()
                .min_by(|a, b| a.1.partial_cmp(b.1).unwrap()) // Use `partial_cmp` for floats
                .map(|(idx, _)| idx)
                .unwrap();
            let box_ = roll_rows(&box_, 4 - startidx as isize);
            scores[index] = score;
            boxes
                .slice_mut(s![index, .., ..])
                .assign(&box_.mapv(|x| x as i64));
        }

        (Some(boxes), Some(scores))
    }

    // default [1.8]
    fn unclip(box_: Vec<(f32, f32)>, unclip_ratio: f64) -> Vec<Vec<(f64, f64)>> {
        let box_ = Path::new(
            box_.into_iter()
                .map(|(x, y)| clipper2::Point::new(x as f64, y as f64))
                .collect(),
        );

        let scaled = box_.scale(100.0, 100.0);
        let length: f64 = scaled
            .iter()
            .zip(scaled.iter().cycle().skip(1))
            .take(scaled.len())
            .map(|(a, b)| a.distance_to(b))
            .sum();
        let area = scaled.signed_area() * unclip_ratio / length;
        let offset = ClipperOffset::new(ClipperOffsetConfig::new(2.0, 0.25, false, false));
        offset.add_path(box_, ClipperJoinType_ROUND_JOIN, ClipperEndType_POLYGON_END);
        let temp = offset.execute(area);
        let v = temp
            .into_iter()
            .map(|v| v.into_iter().map(|v| (v.x(), v.y())).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        v
    }
}

impl Default for SegDetectorRepresenter {
    fn default() -> Self {
        Self {
            min_size: 3.0,
            thresh: 0.6,
            box_thresh: 0.8,
            max_candidates: 1000,
            unclip_ratio: 2.2,
        }
    }
}
