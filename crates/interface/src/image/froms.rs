use ndarray::{Array2, Array3};
use opencv::core::{Mat, MatTraitConst as _};

use crate::{
    detectors::Mask,
    image::{DimType, RawImage},
};

impl From<Array3<f32>> for RawImage {
    fn from(input: Array3<f32>) -> Self {
        Self::from(input.mapv(|v| {
            let clamped = v.clamp(0.0, 1.0);
            (clamped * 255.0) as u8
        }))
    }
}

impl From<Mat> for RawImage {
    fn from(value: Mat) -> Self {
        let resized = if value.is_continuous() {
            value
        } else {
            value.clone()
        };

        let size = resized.size().unwrap();
        let rows = size.height as usize;
        let cols = size.width as usize;
        let channels = resized.channels() as usize;

        let total_len = rows * cols * channels;
        let data: &[u8] = unsafe { std::slice::from_raw_parts(resized.data(), total_len) };

        Self {
            data: data.to_vec(),
            width: cols as DimType,
            height: rows as DimType,
            channels: channels as u8,
        }
    }
}

impl From<Mask> for RawImage {
    fn from(mask: Mask) -> Self {
        RawImage {
            data: mask.data.into_iter().flat_map(|v| vec![v, v, v]).collect(),
            width: mask.width,
            height: mask.height,
            channels: 3,
        }
    }
}

impl From<Array2<u8>> for RawImage {
    fn from(mask: Array2<u8>) -> Self {
        let (height, width) = mask.dim();
        let channels = 3;
        let mut rgb = Array3::<u8>::zeros((height, width, channels));
        for ((row, col), &val) in mask.indexed_iter() {
            rgb[[row, col, 0]] = val;
            rgb[[row, col, 1]] = val;
            rgb[[row, col, 2]] = val;
        }
        let data = if rgb.is_standard_layout() {
            rgb.as_slice().unwrap().to_vec()
        } else {
            rgb.into_iter().collect()
        };

        RawImage {
            data,
            width: width as u16,
            height: height as u16,
            channels: 3,
        }
    }
}

impl From<Array3<u8>> for RawImage {
    fn from(value: Array3<u8>) -> Self {
        let (height, width, channels) = value.dim();

        let data = if value.is_standard_layout() {
            value.as_slice().unwrap().to_vec()
        } else {
            value.into_iter().collect()
        };

        RawImage {
            data,
            width: width as u16,
            height: height as u16,
            channels: channels as u8,
        }
    }
}

impl From<Array2<f32>> for RawImage {
    fn from(input: Array2<f32>) -> Self {
        let (height, width) = input.dim();

        let mut output = Array3::<u8>::zeros((height, width, 3));

        for ((row, col), &val) in input.indexed_iter() {
            let pixel = (val.clamp(0.0, 1.0) * 255.0) as u8;

            output[[row, col, 0]] = pixel;
            output[[row, col, 1]] = pixel;
            output[[row, col, 2]] = pixel;
        }

        Self::from(output)
    }
}
