use crate::{detectors::Mask, image::RawImage, model::Model};

pub trait Inpainter: Model {
    type Options;

    fn inpaint(
        &self,
        image: RawImage,
        mask: &Mask,
        options: Self::Options,
    ) -> anyhow::Result<RawImage>;
}
