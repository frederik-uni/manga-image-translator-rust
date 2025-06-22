use std::{
    fmt::{self},
    path::Path,
};

mod cpu;
pub mod dummy;
mod froms;
#[cfg(feature = "gpu")]
mod gpu;
mod rayon;

pub use cpu::CpuImageProcessor;
#[cfg(feature = "gpu")]
pub use gpu::GpuImageProcessor;
pub use rayon::RayonImageProcessor;

use crate::detectors::Mask;
use ndarray::{Array, Dim};
use opencv::core::{Mat, MatTraitConst as _};

#[cfg(feature = "debug")]
use crate::detectors::textlines::Quadrilateral;
#[cfg(feature = "u16-dims")]
pub type DimType = u16;
#[cfg(not(feature = "u16-dims"))]
pub type DimType = u32;

#[derive(PartialEq, Eq, Clone)]
/// A rgb image
pub struct RawImage {
    pub data: Vec<u8>,
    pub width: DimType,
    pub height: DimType,
    /// Always 3
    pub channels: u8,
}

#[cfg(feature = "debug")]
impl RawImage {
    pub fn draw_bbox(&mut self, textlines: &[Quadrilateral]) {
        use tiny_skia::{Color, Paint, Pixmap, Stroke};

        let rgb_img = self.clone().to_image();
        let mut pixmap = Pixmap::new(self.width as u32, self.height as u32)
            .ok_or("Failed to create Pixmap")
            .unwrap();

        for (x, y, pixel) in rgb_img.enumerate_pixels() {
            let i = (y * self.width as u32 + x) as usize;
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];
            pixmap.pixels_mut()[i] =
                tiny_skia::PremultipliedColorU8::from_rgba(r, g, b, 255).unwrap();
        }

        let mut paint = Paint::default();
        paint.set_color(Color::from_rgba(1.0, 0.0, 0.0, 1.0).unwrap()); // Red
        let stroke = Stroke {
            width: 2.0,
            ..Default::default()
        };

        for txt in textlines {
            use tiny_skia::PathBuilder;

            let mut pb = PathBuilder::new();
            if let Some(&(x0, y0)) = txt.pts().first() {
                pb.move_to(x0 as f32, y0 as f32);
                for &(x, y) in &txt.pts()[1..] {
                    pb.line_to(x as f32, y as f32);
                }
                pb.close();
                let path = pb.finish().unwrap();
                pixmap.stroke_path(
                    &path,
                    &paint,
                    &stroke,
                    tiny_skia::Transform::identity(),
                    None,
                );
            }
        }

        let png_bytes = pixmap.encode_png().unwrap();
        std::fs::write("./img.png", png_bytes).unwrap();
    }

    pub fn display(&self) {
        use show_image::{ImageView, create_window};
        let window = create_window("Image", Default::default()).unwrap();

        let image = ImageView::new(
            show_image::ImageInfo::rgb8(self.width as u32, self.height as u32),
            &self.data,
        );

        window.set_image("frame-0", image).unwrap();

        println!("Press Enter to close the window...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
    }
}

impl fmt::Debug for RawImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RawImage")
            .field("data_len", &self.data.len())
            .field("width", &self.width)
            .field("height", &self.height)
            .field("channels", &self.channels)
            .finish()
    }
}

impl RawImage {
    pub fn to_ndarray(self) -> Array<u8, Dim<[usize; 3]>> {
        Array::from_shape_vec(
            Dim([
                self.height as usize,
                self.width as usize,
                self.channels as usize,
            ]),
            self.data.clone(),
        )
        .unwrap()
    }

    pub fn as_opencv_mat<'a>(&'a self) -> Mat {
        let mat = Mat::from_slice(&self.data).unwrap();
        let mat = mat
            .reshape(self.channels as i32, self.height as i32)
            .unwrap()
            .clone_pointee();
        mat
    }

    pub fn to_image(self) -> image::RgbImage {
        #[cfg(feature = "u16-dims")]
        return image::RgbImage::from_raw(self.width as u32, self.height as u32, self.data)
            .unwrap();
        #[cfg(not(feature = "u16-dims"))]
        image::RgbImage::from_raw(self.width, self.height, self.data).unwrap()
    }

    pub fn new<P: AsRef<Path>>(path: P) -> Result<RawImage, Box<dyn std::error::Error>> {
        let v = path.as_ref();
        let path = if v.is_relative() {
            base_util::project::root_path().join(v)
        } else {
            v.to_path_buf()
        };

        let img = image::open(path)?;

        let rgb_img = img.to_rgb8();

        let (width, height) = rgb_img.dimensions();
        let data = rgb_img.into_raw();
        #[cfg(feature = "u16-dims")]
        let (width, height) = (width as u16, height as u16);
        Ok(RawImage {
            data,
            width,
            height,
            channels: 3,
        })
    }
}

pub trait ImageOp {
    fn invert(&self, image: RawImage) -> RawImage;
    fn add_border(&self, image: RawImage, target_side_length: DimType) -> RawImage {
        self.add_border_wh(image, target_side_length, target_side_length)
    }
    fn add_border_wh(&self, image: RawImage, width: DimType, height: DimType) -> RawImage;
    fn add_border_center(&self, image: RawImage, target_side_length: DimType) -> RawImage;
    fn remove_border(&self, image: RawImage, width: DimType, height: DimType) -> RawImage;
    fn remove_border_center(&self, image: RawImage, width: DimType, height: DimType) -> RawImage;
    fn rotate_right(&self, image: RawImage) -> RawImage;
    fn rotate_left(&self, image: RawImage) -> RawImage;
    fn rotate_left_mask(&self, mask: Mask) -> Mask;
    fn gamma_correction(&self, image: RawImage) -> RawImage;
    fn histogram_equalization(&self, image: RawImage) -> RawImage;
    fn resize(
        &self,
        image: RawImage,
        width: DimType,
        height: DimType,
        interpolation: Interpolation,
    ) -> RawImage;
    fn resize_mask(
        &self,
        image: Vec<u8>,
        old_width: usize,
        old_height: usize,
        width: usize,
        height: usize,
        interpolation: Interpolation,
    ) -> Vec<u8>;

    fn remove_border_mask(&self, mask: Mask, width: DimType, height: DimType) -> Mask;
}
pub enum Interpolation {
    Nearest,
    Box,
    Bilinear,
    Bicubic,
    Lanczos3,
}
