use crate::{detectors::textlines::Quadrilateral, image::RawImage};

pub trait Ocr {
    type Options;
    fn detect(
        &self,
        image: &RawImage,
        areas: &[Quadrilateral],
        options: Self::Options,
    ) -> anyhow::Result<Vec<Quadrilateral>>;
}
