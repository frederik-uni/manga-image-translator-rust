use crate::image::RawImage;

pub trait Upscaler {
    type Options;
    fn upscale(&self, image: &RawImage, options: Self::Options) -> RawImage;
}
