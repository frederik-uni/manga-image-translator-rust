use log::{debug, info};

use crate::{
    detectors::{Mask, PreprocessorOptions, Quadrilateral},
    image::{ImageOp, RawImage},
};

pub fn detect(
    image: &RawImage,
    options: &PreprocessorOptions,
    img_processor: &Box<dyn ImageOp + Send + Sync>,
    callback: impl FnOnce(RawImage) -> anyhow::Result<(Vec<Quadrilateral>, Mask)>,
) -> anyhow::Result<Option<(Vec<Quadrilateral>, Mask)>> {
    let img_h = image.height as i64;
    // Automatically add border if image too small (instead of simply resizing due to them more likely containing large fonts)
    let mut add_border = None;
    if image.width.min(image.height) < 400 {
        add_border = Some((image.width, image.height));
        debug!("Adding border")
    }
    let mut img = img_processor.add_border(image.clone(), 400);
    if options.rotate {
        debug!("Rotating image");
        img = img_processor.rotate_right(img);
    }

    if options.invert {
        debug!("Adding inversion");
        img = img_processor.invert(img);
    }

    if options.gamma_correct {
        debug!("Adding gamma correction");
        img = img_processor.gamma_correction(img);
    }

    let (mut textlines, mut mask) = callback(img)?;

    if options.auto_rotate {
        let rerun = if !textlines.is_empty() {
            textlines.len() * 2 >= textlines.iter().map(|v| v.aspect_ratio() > 1.0).count()
        } else {
            true
        };

        if rerun {
            info!("Rerunning detection with 90° rotation");
            return Ok(None);
        }
    }

    if let Some((w, h)) = add_border {
        debug!("Removing border from mask");

        mask = img_processor.remove_border_mask(mask, w, h);
    }

    if options.rotate {
        debug!("Rotating mask and textlines");
        mask = img_processor.rotate_left_mask(mask);
        textlines = textlines
            .into_iter()
            .map(|v| {
                Quadrilateral::new(
                    v.pts()
                        .iter()
                        .map(|&(x, y)| {
                            let new_x = y;
                            let new_y = -x + img_h;
                            (new_x, new_y)
                        })
                        .collect(),
                    v.score(),
                )
            })
            .collect::<Vec<_>>();
    }
    Ok(Some((textlines, mask)))
}
