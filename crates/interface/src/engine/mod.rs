use std::{
    any::Any,
    sync::{Arc, Mutex},
};

use crate::{
    colorizer::Colorizer,
    detectors::{Detector, PreprocessorOptions},
    image::{ImageOp, RawImage},
    inpainter::Inpainter,
    ocr::Ocr,
    translator::Translator,
    upcaler::Upscaler,
};

struct Engine<D: Detector, OCR: Ocr, I: Inpainter, T: Translator, C: Colorizer, U: Upscaler> {
    detector: Arc<Mutex<D>>,
    ocr: Arc<Mutex<OCR>>,
    translator: Arc<Mutex<T>>,
    colorizer: Arc<Mutex<C>>,
    upscaler: Arc<Mutex<U>>,
    inpainter: Arc<Mutex<I>>,
    options: Options<OCR, I, T, C, U>,
    image_op: Arc<Box<dyn ImageOp + Sync + Send>>,
}

struct Options<OCR: Ocr, I: Inpainter, T: Translator, C: Colorizer, U: Upscaler> {
    pre_detector: PreprocessorOptions,
    detector: Vec<u8>,
    ocr: OCR::Options,
    translator: T::Options,
    inpainter: I::Options,
    upscaler: U::Options,
    colorizer: C::Options,
}

impl<D: Detector, OCR: Ocr, I: Inpainter, T: Translator, C: Colorizer, U: Upscaler>
    Engine<D, OCR, I, T, C, U>
{
    fn call(self, image: RawImage) -> anyhow::Result<Self> {
        let (areas, mask) = self.detector.lock().unwrap().detect(
            &image,
            self.options.pre_detector,
            &self.options.detector,
            &*self.image_op,
        )?;
        let texts = self
            .ocr
            .lock()
            .unwrap()
            .detect(&image, &areas, self.options.ocr)?;
        // let translations = self
        //     .translator
        //     .lock()
        //     .unwrap()
        //     .translate(&texts, self.options.translator)?;
        let inpainted =
            self.inpainter
                .lock()
                .unwrap()
                .inpaint(image, &mask, self.options.inpainter)?;

        todo!()
    }
}
