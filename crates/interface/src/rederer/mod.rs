use crate::{detectors::textlines::Quadrilateral, image::RawImage};

pub trait Renderer {
    type Options;
    fn render(
        &self,
        image: RawImage,
        translations: Quadrilateral,
        options: Self::Options,
    ) -> anyhow::Result<Box<dyn Output>>;
}

pub trait Output {
    fn to_bytes(&self) -> Vec<u8>;

    fn to_file(&self, path: &str) -> anyhow::Result<()>;

    fn join(self: Box<Self>, other: Box<dyn Output>) -> Box<dyn Output>;
}
