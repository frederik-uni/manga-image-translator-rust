use fast_image_resize::{FilterType, ResizeAlg, ResizeOptions, Resizer, images::Image};

use rayon::{
    iter::{
        IndexedParallelIterator as _, IntoParallelIterator as _, IntoParallelRefIterator as _,
        ParallelIterator as _,
    },
    slice::{ParallelSlice as _, ParallelSliceMut as _},
};

use crate::{
    detectors::Mask,
    image::{DimType, ImageOp, Interpolation, RawImage},
};

#[derive(Default)]
pub struct RayonImageProcessor;

impl ImageOp for RayonImageProcessor {
    fn invert(&self, mut image: super::RawImage) -> super::RawImage {
        image
            .data
            .par_chunks_mut(image.channels as usize)
            .for_each(|px| {
                px.iter_mut().for_each(|c| *c = !*c);
            });
        image
    }

    fn add_border_wh(
        &self,
        image: super::RawImage,
        width: DimType,
        height: DimType,
    ) -> super::RawImage {
        let old_w = image.width;
        let old_h = image.height;
        let channels: u32 = 3;

        if old_w > width && old_h > height {
            return image;
        }

        let width = width.max(old_w);
        let height = height.max(old_h);

        let channels_usize = channels as usize;
        let mut new_data = Vec::with_capacity(width as usize * height as usize * channels_usize);
        new_data.resize(new_data.capacity(), 0);

        new_data
            .par_chunks_mut(width as usize * channels_usize)
            .zip(image.data.par_chunks((old_w as u32 * channels) as usize))
            .take(old_h as usize)
            .for_each(|(dst_row, src_row)| {
                dst_row[..src_row.len()].copy_from_slice(src_row);
            });

        super::RawImage {
            data: new_data,
            width,
            height,
            channels: 3,
        }
    }

    fn add_border_center(
        &self,
        image: super::RawImage,
        target_side_length: DimType,
    ) -> super::RawImage {
        let old_w = image.width;
        let old_h = image.height;
        let channels: u32 = 3;

        let new_side = old_w.max(old_h);
        if new_side >= target_side_length {
            return image;
        }
        let new_side = target_side_length;
        let offset_x = (new_side - old_w) / 2;
        let offset_y = (new_side - old_h) / 2;

        let new_side_usize = new_side as usize;
        let channels_usize = channels as usize;
        let mut new_data = Vec::with_capacity(new_side_usize * new_side_usize * channels_usize);
        new_data.resize(new_data.capacity(), 0);

        new_data
            .par_chunks_mut(new_side_usize * channels_usize)
            .skip(offset_y as usize)
            .zip(image.data.par_chunks((old_w as u32 * channels) as usize))
            .take(old_h as usize)
            .for_each(|(dst_row, src_row)| {
                let start = offset_x as usize * channels_usize;
                dst_row[start..start + src_row.len()].copy_from_slice(src_row);
            });

        super::RawImage {
            data: new_data,
            width: new_side,
            height: new_side,
            channels: 3,
        }
    }

    fn remove_border(
        &self,
        image: super::RawImage,
        width: DimType,
        height: DimType,
    ) -> super::RawImage {
        let old_w = image.width as usize;
        let channels = image.channels as usize;

        let new_w = width as usize;
        let new_h = height as usize;

        let mut new_data = Vec::with_capacity(new_w * new_h * channels);
        new_data.resize(new_data.capacity(), 0);

        new_data
            .par_chunks_mut(new_w * channels)
            .zip(image.data.par_chunks(old_w * channels))
            .take(new_h)
            .for_each(|(dst_row, src_row)| {
                dst_row.copy_from_slice(&src_row[..new_w * channels]);
            });

        super::RawImage {
            data: new_data,
            width: new_w as DimType,
            height: new_h as DimType,
            channels: image.channels,
        }
    }

    fn remove_border_center(
        &self,
        image: super::RawImage,
        width: DimType,
        height: DimType,
    ) -> super::RawImage {
        let old_w = image.width as usize;
        let old_h = image.height as usize;
        let channels = image.channels as usize;

        let new_w = width as usize;
        let new_h = height as usize;

        let offset_x = (old_w - new_w) / 2;
        let offset_y = (old_h - new_h) / 2;

        let mut new_data = Vec::with_capacity(new_w * new_h * channels);
        new_data.resize(new_data.capacity(), 0);

        new_data
            .par_chunks_mut(new_w * channels)
            .zip((offset_y..offset_y + new_h).into_par_iter())
            .for_each(|(dst_row, src_row_idx)| {
                let src_start = (src_row_idx * old_w + offset_x) * channels;
                let src_end = src_start + new_w * channels;
                dst_row.copy_from_slice(&image.data[src_start..src_end]);
            });

        super::RawImage {
            data: new_data,
            width: new_w as DimType,
            height: new_h as DimType,
            channels: image.channels,
        }
    }

    fn rotate_right(&self, image: super::RawImage) -> super::RawImage {
        let super::RawImage {
            data,
            width,
            height,
            channels,
        } = image;
        let channels_u = channels as usize;
        let width_u = width as usize;
        let height_u = height as usize;

        let mut rotated_data: Vec<u8> = Vec::with_capacity(width_u * height_u * channels_u);
        unsafe { rotated_data.set_len(rotated_data.capacity()) };

        rotated_data
            .par_chunks_mut(height_u * channels_u)
            .enumerate()
            .for_each(|(dst_r, dst_chunk)| {
                for dst_c in 0..height_u {
                    let r = height_u - 1 - dst_c;
                    let c = dst_r;
                    let src_offset = (r * width_u + c) * channels_u;
                    let dst_offset = dst_c * channels_u;

                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            data.as_ptr().add(src_offset),
                            dst_chunk.as_mut_ptr().add(dst_offset),
                            channels_u,
                        );
                    }
                }
            });

        super::RawImage {
            data: rotated_data,
            width: height,
            height: width,
            channels,
        }
    }

    fn rotate_left(&self, image: super::RawImage) -> super::RawImage {
        let super::RawImage {
            data,
            width,
            height,
            channels,
        } = image;
        let channels_u = channels as usize;
        let width_u = width as usize;
        let height_u = height as usize;

        let mut rotated_data: Vec<u8> = Vec::with_capacity(width_u * height_u * channels_u);
        unsafe { rotated_data.set_len(rotated_data.capacity()) };

        rotated_data
            .par_chunks_mut(height_u * channels_u)
            .enumerate()
            .for_each(|(dst_r, dst_chunk)| {
                for dst_c in 0..height_u {
                    let r = dst_c;
                    let c = width_u - 1 - dst_r;

                    let src_offset = (r * width_u + c) * channels_u;
                    let dst_offset = dst_c * channels_u;

                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            data.as_ptr().add(src_offset),
                            dst_chunk.as_mut_ptr().add(dst_offset),
                            channels_u,
                        );
                    }
                }
            });

        super::RawImage {
            data: rotated_data,
            width: height,
            height: width,
            channels,
        }
    }

    fn gamma_correction(&self, image: super::RawImage) -> super::RawImage {
        let mid = 0.5;
        let pixel_count = (image.width as u64) * (image.height as u64);

        let sum_luminance = image
            .data
            .par_chunks(3)
            .map(|pix| {
                let b = pix[0] as f64;
                let g = pix[1] as f64;
                let r = pix[2] as f64;
                0.114 * b + 0.587 * g + 0.299 * r
            })
            .sum::<f64>();

        let mean = sum_luminance / (pixel_count as f64);
        let temp: f64 = mid * 255.0;
        let gamma = temp.ln() / mean.ln();
        let lut: Vec<u8> = (0..=255)
            .map(|val| {
                let normalized = (val as f64) / 255.0;
                let corrected = 255.0 * normalized.powf(gamma);
                corrected.max(0.0).min(255.0).round() as u8
            })
            .collect();

        let corrected_data: Vec<u8> = image
            .data
            .par_iter()
            .map(|&val| lut[val as usize])
            .collect();

        super::RawImage {
            data: corrected_data,
            width: image.width,
            height: image.height,
            channels: image.channels,
        }
    }

    fn histogram_equalization(&self, image: super::RawImage) -> super::RawImage {
        let mut output = Vec::with_capacity(image.data.len());
        unsafe { output.set_len(output.capacity()) };

        let size = image.width as u32 * image.height as u32;

        // Step 1: Convert RGB to YUV and extract Y channel
        let mut y_channel = Vec::with_capacity(size as usize);
        unsafe { y_channel.set_len(y_channel.capacity()) };

        let mut u_channel = Vec::with_capacity(size as usize);
        unsafe { u_channel.set_len(u_channel.capacity()) };

        let mut v_channel = Vec::with_capacity(size as usize);
        unsafe { v_channel.set_len(v_channel.capacity()) };
        let mut hist = [0u32; 256];

        image
            .data
            .par_chunks_exact(3)
            .enumerate()
            .map(|(i, chunk)| {
                let r = chunk[0] as f32;
                let g = chunk[1] as f32;
                let b = chunk[2] as f32;

                let y = (0.299 * r + 0.587 * g + 0.114 * b)
                    .round()
                    .clamp(0.0, 255.0) as u8;
                let u = ((-0.169 * r - 0.331 * g + 0.5 * b) + 128.0)
                    .round()
                    .clamp(0.0, 255.0) as u8;
                let v = ((0.5 * r - 0.419 * g - 0.081 * b) + 128.0)
                    .round()
                    .clamp(0.0, 255.0) as u8;

                (i, y, u, v)
            })
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|(i, y, u, v)| {
                y_channel[i] = y;
                u_channel[i] = u;
                v_channel[i] = v;
                hist[y as usize] += 1;
            });

        // Compute cumulative distribution function (CDF)
        let mut cdf = [0u32; 256];
        let mut cdf_min = u32::MAX;
        let mut cumsum = 0;
        for (i, &count) in hist.iter().enumerate() {
            cumsum += count;
            cdf[i] = cumsum;
            if count != 0 && cdf_min == u32::MAX {
                cdf_min = cumsum;
            }
        }

        let total_pixels = size as u32;
        let scale = 255.0 / (total_pixels - cdf_min).max(1) as f32;

        let mut lut = [0u8; 256];
        for i in 0..256 {
            let cdf_value = cdf[i];
            lut[i] = (((cdf_value - cdf_min).max(0) as f32 * scale)
                .round()
                .clamp(0.0, 255.0)) as u8;
        }

        let equalized_y: Vec<u8> = y_channel.par_iter().map(|&y| lut[y as usize]).collect();

        // Step 3: Convert YUV back to RGB
        output.par_chunks_mut(3).enumerate().for_each(|(i, chunk)| {
            let y = equalized_y[i] as f32;
            let u = u_channel[i] as f32 - 128.0;
            let v = v_channel[i] as f32 - 128.0;

            let r = (y + 1.402 * v).round().clamp(0.0, 255.0) as u8;
            let g = (y - 0.344136 * u - 0.714136 * v).round().clamp(0.0, 255.0) as u8;
            let b = (y + 1.772 * u).round().clamp(0.0, 255.0) as u8;

            chunk[0] = r;
            chunk[1] = g;
            chunk[2] = b;
        });

        super::RawImage {
            data: output,
            width: image.width,
            height: image.height,
            channels: 3,
        }
    }

    fn resize(
        &self,
        image: super::RawImage,
        width: DimType,
        height: DimType,
        interpolation: Interpolation,
    ) -> super::RawImage {
        let resize_alg = match interpolation {
            Interpolation::Nearest => ResizeAlg::Nearest,
            Interpolation::Box => ResizeAlg::Convolution(FilterType::Box),
            Interpolation::Bilinear => ResizeAlg::Convolution(FilterType::Bilinear),
            Interpolation::Bicubic => ResizeAlg::Convolution(FilterType::CatmullRom),
            Interpolation::Lanczos3 => ResizeAlg::Convolution(FilterType::Lanczos3),
        };
        let mut resizer = Resizer::new();
        let src_image = Image::from_vec_u8(
            image.width as u32,
            image.height as u32,
            image.data,
            fast_image_resize::PixelType::U8x3,
        )
        .unwrap();
        let mut dst_image = Image::new(
            width as u32,
            height as u32,
            fast_image_resize::PixelType::U8x3,
        );
        resizer
            .resize(
                &src_image,
                &mut dst_image,
                Some(&ResizeOptions::new().use_alpha(false).resize_alg(resize_alg)),
            )
            .unwrap();
        super::RawImage {
            width: dst_image.width() as DimType,
            height: dst_image.height() as DimType,
            channels: 3,
            data: dst_image.into_vec(),
        }
    }

    fn resize_mask(
        &self,
        image: Mask,
        width: usize,
        height: usize,
        interpolation: Interpolation,
    ) -> Mask {
        let resize_alg = match interpolation {
            Interpolation::Nearest => ResizeAlg::Nearest,
            Interpolation::Box => ResizeAlg::Convolution(FilterType::Box),
            Interpolation::Bilinear => ResizeAlg::Convolution(FilterType::Bilinear),
            Interpolation::Bicubic => ResizeAlg::Convolution(FilterType::CatmullRom),
            Interpolation::Lanczos3 => ResizeAlg::Convolution(FilterType::Lanczos3),
        };
        let mut resizer = Resizer::new();
        let src_image = Image::from_vec_u8(
            image.width as u32,
            image.height as u32,
            image.data,
            fast_image_resize::PixelType::U8,
        )
        .unwrap();
        let mut dst_image = Image::new(
            width as u32,
            height as u32,
            fast_image_resize::PixelType::U8,
        );
        resizer
            .resize(
                &src_image,
                &mut dst_image,
                Some(&ResizeOptions::new().use_alpha(false).resize_alg(resize_alg)),
            )
            .unwrap();
        Mask {
            data: dst_image.into_vec(),
            width: width as DimType,
            height: height as DimType,
        }
    }
    fn remove_border_mask(&self, mask: Mask, width: DimType, height: DimType) -> Mask {
        let mut cropped = vec![0u8; width as usize * height as usize];

        cropped
            .par_chunks_mut(width as usize)
            .enumerate()
            .for_each(|(y, dst_row)| {
                let src_start = y * mask.width as usize;
                let src_row = &mask.data[src_start..src_start + width as usize];
                dst_row.copy_from_slice(src_row);
            });

        Mask {
            width,
            height,
            data: cropped,
        }
    }

    fn rotate_left_mask(&self, mask: Mask) -> Mask {
        let rotated = self.rotate_left(RawImage {
            data: mask.data,
            width: mask.width,
            height: mask.height,
            channels: 1,
        });
        Mask {
            width: rotated.width,
            height: rotated.height,
            data: rotated.data,
        }
    }

    fn transpose(&self, image: RawImage) -> RawImage {
        let mut output = vec![0u8; image.data.len()];

        output
            .par_chunks_mut(image.height as usize * 3) // each chunk corresponds to one output row
            .enumerate()
            .for_each(|(x, out_row)| {
                for y in 0..image.height as usize {
                    let in_offset = (y * image.width as usize + x) * 3;
                    let out_offset = y * 3;

                    out_row[out_offset..out_offset + 3]
                        .copy_from_slice(&image.data[in_offset..in_offset + 3]);
                }
            });

        RawImage {
            data: output,
            width: image.height,
            height: image.width,
            channels: 3,
        }
    }
}
