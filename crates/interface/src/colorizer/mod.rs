use std::sync::Arc;

use crate::image::RawImage;

pub trait Colorizer {
    type Options;
    fn colorize(
        &self,
        image: Arc<RawImage>,
        options: Self::Options,
    ) -> anyhow::Result<Arc<RawImage>>;
}
