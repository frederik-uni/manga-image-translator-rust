use crate::{
    detectors::Mask,
    image::{DimType, ImageOp},
};

#[derive(Default)]
pub struct DummyImageProcessor;

impl DummyImageProcessor {
    pub fn new() -> Self {
        DummyImageProcessor
    }
}

impl ImageOp for DummyImageProcessor {
    fn invert(&self, image: super::RawImage) -> super::RawImage {
        image
    }

    fn add_border_wh(
        &self,
        img: super::RawImage,
        width: super::DimType,
        height: super::DimType,
    ) -> super::RawImage {
        if img.width > width && img.height > height {
            return img;
        }
        super::RawImage {
            data: vec![0; width as usize * height as usize * 3],
            width,
            height,
            channels: 3,
        }
    }

    fn add_border_center(&self, _: super::RawImage, width: super::DimType) -> super::RawImage {
        super::RawImage {
            data: vec![0; width as usize * width as usize * 3],
            width,
            height: width,
            channels: 3,
        }
    }

    fn remove_border(
        &self,
        _: super::RawImage,
        width: super::DimType,
        height: super::DimType,
    ) -> super::RawImage {
        super::RawImage {
            data: vec![0; width as usize * height as usize * 3],
            width,
            height,
            channels: 3,
        }
    }

    fn remove_border_center(
        &self,
        _: super::RawImage,
        width: super::DimType,
        height: super::DimType,
    ) -> super::RawImage {
        super::RawImage {
            data: vec![0; width as usize * height as usize * 3],
            width,
            height,
            channels: 3,
        }
    }

    fn rotate_right(&self, mut image: super::RawImage) -> super::RawImage {
        let temp = image.height;
        image.height = image.width;
        image.width = temp;
        image
    }

    fn rotate_left(&self, mut image: super::RawImage) -> super::RawImage {
        let temp = image.height;
        image.height = image.width;
        image.width = temp;
        image
    }

    fn rotate_left_mask(&self, mut mask: crate::detectors::Mask) -> crate::detectors::Mask {
        let temp = mask.height;
        mask.height = mask.width;
        mask.width = temp;
        mask
    }

    fn gamma_correction(&self, image: super::RawImage) -> super::RawImage {
        image
    }

    fn histogram_equalization(&self, image: super::RawImage) -> super::RawImage {
        image
    }

    fn resize(
        &self,
        _: super::RawImage,
        width: super::DimType,
        height: super::DimType,
        _: super::Interpolation,
    ) -> super::RawImage {
        super::RawImage {
            data: vec![0; width as usize * height as usize * 3],
            width,
            height,
            channels: 3,
        }
    }

    fn resize_mask(&self, _: Mask, width: usize, height: usize, _: super::Interpolation) -> Mask {
        Mask {
            width: width as DimType,
            height: height as DimType,
            data: vec![0; width * height],
        }
    }

    fn remove_border_mask(
        &self,
        _: crate::detectors::Mask,
        width: super::DimType,
        height: super::DimType,
    ) -> crate::detectors::Mask {
        Mask {
            data: vec![0; width as usize * height as usize],
            width,
            height,
        }
    }
}
