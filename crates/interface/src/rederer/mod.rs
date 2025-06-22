use crate::{detectors::textlines::Quadrilateral, image::RawImage};

pub trait Renderer {
    type Options;
    type Output;
    fn render(
        &self,
        image: RawImage,
        translations: Quadrilateral,
        options: Self::Options,
    ) -> anyhow::Result<Self::Output>;
}
