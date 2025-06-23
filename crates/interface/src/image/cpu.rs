use crate::{
    detectors::Mask,
    image::{DimType, ImageOp, Interpolation, RawImage, RayonImageProcessor},
};

#[derive(Default)]

pub struct CpuImageProcessor;
//todo: packed_simd
impl ImageOp for CpuImageProcessor {
    fn invert(&self, mut image: super::RawImage) -> super::RawImage {
        image.data.iter_mut().for_each(|byte| *byte = !*byte);

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

        let mut new_data = Vec::with_capacity(width as usize * height as usize * channels as usize);
        new_data.resize(new_data.capacity(), 0);

        for row in 0..old_h {
            let partial = row as u32 * channels;
            let src_start = old_w as u32 * partial;
            let dst_start = width as u32 * partial;

            unsafe {
                std::ptr::copy_nonoverlapping(
                    image.data.as_ptr().add(src_start as usize),
                    new_data.as_mut_ptr().add(dst_start as usize),
                    (old_w as u32 * channels) as usize,
                );
            }
        }

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
        let pad_x = (new_side - old_w) / 2;
        let pad_y = (new_side - old_h) / 2;

        let new_side_usize = new_side as usize;
        let mut new_data = Vec::with_capacity(new_side_usize * new_side_usize * channels as usize);
        new_data.resize(new_data.capacity(), 0);

        for row in 0..old_h {
            let partial = row as u32 * channels;
            let src_start = old_w as u32 * partial;
            let dst_start =
                ((row as u32 + pad_y as u32) * new_side as u32 + pad_x as u32) * channels;

            unsafe {
                std::ptr::copy_nonoverlapping(
                    image.data.as_ptr().add(src_start as usize),
                    new_data.as_mut_ptr().add(dst_start as usize),
                    (old_w as u32 * channels) as usize,
                );
            }
        }

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
        let channels: usize = 3;
        let orig_stride = image.width as usize * channels;
        let new_stride = width as usize * channels;

        let mut new_data = Vec::with_capacity(width as usize * height as usize * channels);
        new_data.resize(new_data.capacity(), 0);

        for row in 0..height as usize {
            let src_offset = row * orig_stride;
            let dst_offset = row * new_stride;

            unsafe {
                std::ptr::copy_nonoverlapping(
                    image.data.as_ptr().add(src_offset),
                    new_data.as_mut_ptr().add(dst_offset),
                    new_stride,
                );
            }
        }

        super::RawImage {
            data: new_data,
            width,
            height,
            channels: 3,
        }
    }

    fn remove_border_center(
        &self,
        image: super::RawImage,
        width: DimType,
        height: DimType,
    ) -> super::RawImage {
        let channels: u32 = 3;
        let new_w = width as usize;
        let new_h = height as usize;

        let pad_x = (image.width as usize - new_w) / 2;
        let pad_y = (image.height as usize - new_h) / 2;

        let mut new_data = Vec::with_capacity(new_w * new_h * channels as usize);
        new_data.resize(new_data.capacity(), 0);

        for row in 0..new_h {
            let src_start = ((row + pad_y) * image.width as usize + pad_x) * channels as usize;
            let dst_start = row * new_w * channels as usize;

            unsafe {
                std::ptr::copy_nonoverlapping(
                    image.data.as_ptr().add(src_start as usize),
                    new_data.as_mut_ptr().add(dst_start as usize),
                    new_w * channels as usize,
                );
            }
        }

        super::RawImage {
            data: new_data,
            width: new_w as DimType,
            height: new_h as DimType,
            channels: 3,
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

        let mut rotated_data = Vec::with_capacity(width_u * height_u * channels_u);
        unsafe { rotated_data.set_len(rotated_data.capacity()) };

        unsafe {
            let src_ptr = data.as_ptr();
            let dst_ptr: *mut u8 = rotated_data.as_mut_ptr();

            for r in 0..height_u {
                for c in 0..width_u {
                    let src_offset = (r * width_u + c) * channels_u;
                    let dst_r = c;
                    let dst_c = height_u - 1 - r;
                    let dst_offset = (dst_r * height_u + dst_c) * channels_u;
                    std::ptr::copy_nonoverlapping(
                        src_ptr.add(src_offset),
                        dst_ptr.add(dst_offset),
                        channels_u,
                    );
                }
            }
        }

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

        let mut rotated_data = Vec::with_capacity(width_u * height_u * channels_u);
        unsafe { rotated_data.set_len(rotated_data.capacity()) };

        unsafe {
            let src_ptr = data.as_ptr();
            let dst_ptr: *mut u8 = rotated_data.as_mut_ptr();

            for r in 0..height_u {
                for c in 0..width_u {
                    let src_offset = (r * width_u + c) * channels_u;
                    let dst_r = width_u - 1 - c;
                    let dst_c = r;
                    let dst_offset = (dst_r * height_u + dst_c) * channels_u;

                    for ch in 0..channels_u {
                        *dst_ptr.add(dst_offset + ch) = *src_ptr.add(src_offset + ch);
                    }
                }
            }
        }

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

        let data = &image.data;
        let mut sum_luminance = 0f64;

        for i in (0..data.len()).step_by(3) {
            let b = data[i] as f64;
            let g = data[i + 1] as f64;
            let r = data[i + 2] as f64;
            sum_luminance += 0.114 * b + 0.587 * g + 0.299 * r;
        }

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

        let mut corrected_data = Vec::with_capacity(data.len());
        for &val in data.iter() {
            corrected_data.push(lut[val as usize]);
        }

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

        for (i, chunk) in image.data.chunks_exact(3).enumerate() {
            let i = i as usize;
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

            y_channel[i] = y;
            u_channel[i] = u;
            v_channel[i] = v;

            hist[y as usize] += 1;
        }

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

        let equalized_y: Vec<u8> = y_channel.iter().map(|&y| lut[y as usize]).collect();

        // Step 3: Convert YUV back to RGB
        for i in 0..size {
            let i = i as usize;
            let y = equalized_y[i] as f32;
            let u = u_channel[i] as f32 - 128.0;
            let v = v_channel[i] as f32 - 128.0;

            let r = (y + 1.402 * v).round().clamp(0.0, 255.0) as u8;
            let g = (y - 0.344136 * u - 0.714136 * v).round().clamp(0.0, 255.0) as u8;
            let b = (y + 1.772 * u).round().clamp(0.0, 255.0) as u8;

            output[i * 3] = r;
            output[i * 3 + 1] = g;
            output[i * 3 + 2] = b;
        }

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
        RayonImageProcessor::default().resize(image, width, height, interpolation)
    }

    fn resize_mask(
        &self,
        image: Mask,
        width: usize,
        height: usize,
        interpolation: Interpolation,
    ) -> Mask {
        RayonImageProcessor::default().resize_mask(image, width, height, interpolation)
    }

    fn remove_border_mask(&self, mask: Mask, width: DimType, height: DimType) -> Mask {
        let mut cropped = vec![0u8; width as usize * height as usize];
        for y in 0..height as usize {
            let src_start = y * mask.width as usize;
            let dst_start = y * width as usize;
            cropped[dst_start..dst_start + width as usize]
                .copy_from_slice(&mask.data[src_start..src_start + width as usize]);
        }
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
}
