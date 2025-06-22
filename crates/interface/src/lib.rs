pub mod colorizer;
pub mod detectors;
pub mod engine;
pub mod image;
pub mod inpainter;
pub mod model;
mod models;
pub mod ocr;
pub mod rederer;
pub mod translator;
pub mod upcaler;

#[cfg(test)]
mod tests {
    mod invert {
        use crate::image::{CpuImageProcessor, ImageOp, RawImage, RayonImageProcessor};

        #[test]
        fn invert_cpu() {
            let image = RawImage::new("imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.png")
                .expect("Failed to load image");
            let inverted =
                RawImage::new("imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.inverted.png")
                    .expect("Failed to load image");
            let image = CpuImageProcessor::default().invert(image);
            assert_eq!(image, inverted)
        }

        #[test]
        fn invert_rayon() {
            let image = RawImage::new("imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.png")
                .expect("Failed to load image");
            let inverted =
                RawImage::new("imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.inverted.png")
                    .expect("Failed to load image");
            let image = RayonImageProcessor::default().invert(image);
            assert_eq!(image, inverted)
        }

        #[cfg(feature = "gpu")]
        #[test]
        fn invert_gpu() {
            let image = RawImage::new("imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.png")
                .expect("Failed to load image");
            let inverted =
                RawImage::new("imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.inverted.png")
                    .expect("Failed to load image");
            let image = crate::image::GpuImageProcessor::new().invert(image);
            assert_eq!(image, inverted)
        }
    }

    mod add_border {
        use crate::image::{CpuImageProcessor, ImageOp, RawImage, RayonImageProcessor};
        #[test]
        fn cpu() {
            let image_src =
                RawImage::new("imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.png")
                    .expect("Failed to load image");
            let border =
                RawImage::new("imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.border.png")
                    .expect("Failed to load image");
            let image = CpuImageProcessor::default().add_border(image_src.clone(), 3000);
            assert_eq!(image, border);
            let image = CpuImageProcessor::default().remove_border(
                image,
                image_src.width,
                image_src.height,
            );

            assert_eq!(image, image_src)
        }

        #[test]
        fn rayon() {
            let image_src =
                RawImage::new("imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.png")
                    .expect("Failed to load image");
            let border =
                RawImage::new("imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.border.png")
                    .expect("Failed to load image");
            let image = RayonImageProcessor::default().add_border(image_src.clone(), 3000);
            assert_eq!(image, border);
            let image = RayonImageProcessor::default().remove_border(
                image,
                image_src.width,
                image_src.height,
            );

            assert_eq!(image, image_src)
        }

        #[cfg(feature = "gpu")]
        #[test]
        fn gpu() {
            let image_src =
                RawImage::new("imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.png")
                    .expect("Failed to load image");
            let border =
                RawImage::new("imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.border.png")
                    .expect("Failed to load image");
            let gpu = crate::image::GpuImageProcessor::new();
            let image = gpu.add_border(image_src.clone(), 3000);
            assert_eq!(image, border);
            let image = gpu.remove_border(image, image_src.width, image_src.height);

            assert_eq!(image, image_src)
        }
    }

    mod add_border_center {
        use crate::image::{CpuImageProcessor, ImageOp, RawImage, RayonImageProcessor};
        #[test]
        fn cpu() {
            let image_src =
                RawImage::new("imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.png")
                    .expect("Failed to load image");
            let border = RawImage::new(
                "imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.border_center.png",
            )
            .expect("Failed to load image");
            let image = CpuImageProcessor::default().add_border_center(image_src.clone(), 3000);
            assert_eq!(image, border);
            let image = CpuImageProcessor::default().remove_border_center(
                image,
                image_src.width,
                image_src.height,
            );

            assert_eq!(image, image_src)
        }

        #[test]
        fn rayon() {
            let image_src =
                RawImage::new("imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.png")
                    .expect("Failed to load image");
            let border = RawImage::new(
                "imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.border_center.png",
            )
            .expect("Failed to load image");
            let image = RayonImageProcessor::default().add_border_center(image_src.clone(), 3000);
            assert_eq!(image, border);
            let image = RayonImageProcessor::default().remove_border_center(
                image,
                image_src.width,
                image_src.height,
            );

            assert_eq!(image, image_src)
        }

        #[cfg(feature = "gpu")]
        #[test]
        fn gpu() {
            let image_src =
                RawImage::new("imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.png")
                    .expect("Failed to load image");
            let border = RawImage::new(
                "imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.border_center.png",
            )
            .expect("Failed to load image");
            let gpu = crate::image::GpuImageProcessor::new();
            let image = gpu.add_border_center(image_src.clone(), 3000);
            assert_eq!(image, border);
            let image = gpu.remove_border_center(image, image_src.width, image_src.height);

            assert_eq!(image, image_src)
        }
    }

    mod rotate {
        use crate::image::{CpuImageProcessor, ImageOp, RawImage, RayonImageProcessor};
        #[test]
        fn cpu() {
            let image_src =
                RawImage::new("imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.png")
                    .expect("Failed to load image");
            let rotate =
                RawImage::new("imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.rotate.png")
                    .expect("Failed to load image");
            let image = CpuImageProcessor::default().rotate_right(image_src.clone());
            assert_eq!(image, rotate);
            let image = CpuImageProcessor::default().rotate_left(image);
            assert_eq!(image_src, image)
        }

        #[test]
        fn rayon() {
            let image_src =
                RawImage::new("imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.png")
                    .expect("Failed to load image");
            let rotate =
                RawImage::new("imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.rotate.png")
                    .expect("Failed to load image");
            let image = RayonImageProcessor::default().rotate_right(image_src.clone());
            assert_eq!(image, rotate);
            let image = RayonImageProcessor::default().rotate_left(image);
            assert_eq!(image_src, image)
        }

        #[cfg(feature = "gpu")]
        #[test]
        fn gpu() {
            let image_src =
                RawImage::new("imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.png")
                    .expect("Failed to load image");
            let rotate =
                RawImage::new("imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.rotate.png")
                    .expect("Failed to load image");
            let gpu = GpuImageProcessor::new();
            let image = gpu.rotate_right(image_src.clone());
            assert_eq!(image, rotate);
            let image = gpu.rotate_left(image);
            assert_eq!(image_src, image)
        }
    }

    mod gamma_correction {
        use crate::image::{CpuImageProcessor, ImageOp, RawImage, RayonImageProcessor};
        #[test]
        fn cpu() {
            let image = RawImage::new("imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.png")
                .expect("Failed to load image");
            let inverted =
                RawImage::new("imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.gamma.png")
                    .expect("Failed to load image");
            let image = CpuImageProcessor::default().gamma_correction(image);
            assert_eq!(image, inverted)
        }

        #[test]
        fn rayon() {
            let image = RawImage::new("imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.png")
                .expect("Failed to load image");
            let inverted =
                RawImage::new("imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.gamma.png")
                    .expect("Failed to load image");
            let image = RayonImageProcessor::default().gamma_correction(image);
            assert_eq!(image, inverted)
        }

        #[cfg(feature = "gpu")]
        #[test]
        fn gpu() {
            let image = RawImage::new("imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.png")
                .expect("Failed to load image");
            let inverted =
                RawImage::new("imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.gamma.png")
                    .expect("Failed to load image");
            let image = crate::image::GpuImageProcessor::new().gamma_correction(image);
            assert_eq!(image, inverted)
        }
    }

    mod histogram_equalization {
        use crate::image::{CpuImageProcessor, ImageOp, RawImage, RayonImageProcessor};
        #[test]
        fn cpu() {
            let image = RawImage::new("imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.png")
                .expect("Failed to load image");
            let inverted =
                RawImage::new("imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.histogram.png")
                    .expect("Failed to load image");
            let image = CpuImageProcessor::default().histogram_equalization(image);
            assert_eq!(image, inverted)
        }

        #[test]
        fn rayon() {
            let image = RawImage::new("imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.png")
                .expect("Failed to load image");
            let inverted =
                RawImage::new("imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.histogram.png")
                    .expect("Failed to load image");
            let image = RayonImageProcessor::default().histogram_equalization(image);
            assert_eq!(image, inverted)
        }

        #[cfg(feature = "gpu")]
        #[test]
        fn gpu() {
            let image = RawImage::new("imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.png")
                .expect("Failed to load image");
            let inverted =
                RawImage::new("imgs/232265329-6a560438-e887-4f7f-b6a1-a61b8648f781.histogram.png")
                    .expect("Failed to load image");
            let image = crate::image::GpuImageProcessor::new().histogram_equalization(image);
            assert_eq!(image, inverted)
        }
    }
}
