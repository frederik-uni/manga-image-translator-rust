use std::sync::{Arc, Mutex};

use crate::image::{DimType, ImageOp, Interpolation};

pub struct GpuImageProcessor {}

impl GpuImageProcessor {
    pub fn new() -> Self {
        Self {}
    }
}

#[allow(unused_variables)]
impl ImageOp for GpuImageProcessor {
    fn invert(&self, mut image: super::RawImage) -> super::RawImage {
        //TODO: improve by processing 4 u8 at a time
        // let buffer = image.data.into_iter().map(|v| v as u32).collect::<Vec<_>>();
        // let out = self
        //     .device
        //     .lock()
        //     .expect("Mutex poinsoning")
        //     .apply_on_vector(buffer, "255u - element");
        // image.data = out.into_iter().map(|v| v as u8).collect();

        todo!()
    }

    fn add_border_wh(
        &self,
        image: super::RawImage,
        width: DimType,
        height: DimType,
    ) -> super::RawImage {
        todo!()
    }

    fn add_border_center(
        &self,
        image: super::RawImage,
        target_side_length: DimType,
    ) -> super::RawImage {
        todo!()
    }

    fn rotate_right(&self, image: super::RawImage) -> super::RawImage {
        todo!()
    }

    fn rotate_left(&self, image: super::RawImage) -> super::RawImage {
        todo!()
    }

    fn gamma_correction(&self, image: super::RawImage) -> super::RawImage {
        todo!()
    }

    fn histogram_equalization(&self, image: super::RawImage) -> super::RawImage {
        todo!()
    }

    fn remove_border(
        &self,
        image: super::RawImage,
        width: DimType,
        height: DimType,
    ) -> super::RawImage {
        todo!()
    }

    fn remove_border_center(
        &self,
        image: super::RawImage,
        width: DimType,
        height: DimType,
    ) -> super::RawImage {
        todo!()
    }

    fn resize(
        &self,
        image: super::RawImage,
        width: DimType,
        height: DimType,
        interpolation: Interpolation,
    ) -> super::RawImage {
        todo!()
    }
}
