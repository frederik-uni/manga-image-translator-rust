use base_util::error::PreProcessingError;
use interface::image::RawImage;
use opencv::core::{Mat, ToInputArray};

//TODO: refactor + test + bench
pub fn bilateral_filter(
    src: &impl ToInputArray,
    d: i32,
    sigma_color: f64,
    sigma_space: f64,
    border_type: i32,
) -> Result<RawImage, PreProcessingError> {
    let mut filtered = Mat::default();
    opencv::imgproc::bilateral_filter(
        src,
        &mut filtered,
        d,
        sigma_color,
        sigma_space,
        border_type,
    )?;
    Ok(filtered.try_into()?)
}
