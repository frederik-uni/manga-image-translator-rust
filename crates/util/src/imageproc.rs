use std::borrow::Cow;

use interface::image::{DimType, ImageOp, Interpolation, RawImage};
use ndarray::ArrayView2;
use opencv::{
    core::{Mat, MatTraitConst as _, Point, Vector, CV_8UC1},
    imgproc::{self, CHAIN_APPROX_SIMPLE, RETR_LIST},
};

pub fn resize_aspect_ratio(
    img: RawImage,
    square_size: i64,
    interpolation: Interpolation,
    mag_ratio: f64,
    op: &Box<dyn ImageOp + Send + Sync>,
) -> ResizeData {
    let (height, width, _) = (img.height, img.width, img.channels);
    let mut target_size = mag_ratio * square_size as f64;
    if target_size > square_size as f64 {
        target_size = square_size as f64
    }
    let ratio = target_size / height.max(width) as f64;
    let (target_h, target_w) = (
        f64::round(height as f64 * ratio) as i32,
        f64::round(width as f64 * ratio) as i32,
    );

    let proc = op.resize(img, target_w as DimType, target_h as DimType, interpolation);

    const MULT: i32 = 256;

    let (mut target_h32, mut target_w32) = (target_h, target_w);
    let mut pad_h = 0;
    let mut pad_w = 0;
    if target_h % MULT != 0 {
        pad_h = MULT - target_h % MULT;
        target_h32 = target_h + pad_h;
    }
    if target_w % MULT != 0 {
        pad_w = MULT - target_w % MULT;
        target_w32 = target_w + pad_w
    }
    let resized = op.add_border_wh(proc, target_w32 as u16, target_h32 as u16);
    ResizeData {
        img: resized,
        ratio,
        heatmap: (target_w32 / 2, target_h32 / 2),
        pad_w,
        pad_h,
    }
}

pub struct ResizeData {
    pub img: RawImage,
    pub ratio: f64,
    pub heatmap: (i32, i32),
    pub pad_w: i32,
    pub pad_h: i32,
}

/// Finds contours from a binary ndarray image
pub fn find_contours_from_ndarray(
    bitmap: &ArrayView2<bool>,
) -> opencv::Result<Vector<Vector<Point>>> {
    let scaled = bitmap.mapv(|v| if v { 255_u8 } else { 0_u8 });

    let (rows, _) = scaled.dim();

    let scaled = match scaled.as_slice() {
        Some(v) => Cow::Borrowed(v),
        None => Cow::Owned(scaled.into_iter().collect()),
    };

    let mat = Mat::from_slice(scaled.as_ref())?;
    let mat = mat.reshape(CV_8UC1, rows as i32)?;

    let mut contours = Vector::<Vector<Point>>::new();

    imgproc::find_contours(
        &mat,
        &mut contours,
        RETR_LIST,
        CHAIN_APPROX_SIMPLE,
        Point::new(0, 0),
    )?;

    Ok(contours)
}

#[cfg(test)]
mod tests {
    use super::*;
    use interface::image::{CpuImageProcessor, Interpolation, RawImage};
    use ndarray::array;

    #[test]
    fn test_resize_aspect_ratio() {
        let img = RawImage {
            width: 300,
            height: 150,
            channels: 3,
            data: vec![255; 300 * 150 * 3],
        };

        let op: Box<dyn ImageOp + Send + Sync> = Box::new(CpuImageProcessor::default());
        let square_size = 512;
        let mag_ratio = 1.5;
        let interpolation = Interpolation::Nearest;

        let result = resize_aspect_ratio(img, square_size, interpolation, mag_ratio, &op);

        assert!(result.img.width % 256 == 0);
        assert!(result.img.height % 256 == 0);
        assert!(result.ratio > 0.0);
    }

    #[test]
    fn test_find_contours_from_ndarray() {
        let bitmap = array![
            [false, true, true, false],
            [false, true, true, false],
            [false, false, false, false],
            [true, true, false, false],
        ];

        let contours = find_contours_from_ndarray(&bitmap.view()).expect("Failed to find contours");

        assert!(!contours.is_empty());
    }
}
