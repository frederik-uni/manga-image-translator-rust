use std::sync::Arc;

use base_util::onnx::{Providers, all_providers};
use dbnet::{DbNetDetector, DefaultOptions};
use interface::{
    detectors::{Detector, PreprocessorOptions},
    image::{CpuImageProcessor, ImageOp, RawImage},
    model::{CreateData, Model as _},
};
use numpy::{PyArrayMethods, PyReadonlyArray3};
use pyo3::prelude::*;

#[pyclass]
pub struct Session {
    processor: Arc<Box<dyn ImageOp + Send + Sync>>,
    inner: CreateData,
}

#[pymethods]
impl Session {
    #[new]
    /// allowed providers are cuda, coreml, directml, tensorrt
    /// all are enabled by default
    fn new(providers: Option<Vec<String>>) -> Self {
        let providers = match providers {
            None => all_providers(),
            Some(providers) => providers
                .iter()
                .map(|v| match v.as_str() {
                    "cuda" => Providers::CUDA,
                    "coreml" => Providers::CoreML,
                    "directml" => Providers::DirectML,
                    "tensorrt" => Providers::TensorRT,
                    _ => panic!("Invalid provider"),
                })
                .collect(),
        };
        Session {
            inner: CreateData::new(providers),
            processor: Arc::new(Box::new(CpuImageProcessor::default())),
        }
    }

    fn default_detector(&self) -> PyDetector {
        PyDetector {
            inner: Box::new(DbNetDetector::new(self.inner.clone(), false))
                as Box<dyn Detector + Send + Sync>,

            processor: self.processor.clone(),
        }
    }

    fn convnext_detector(&self) -> PyDetector {
        PyDetector {
            inner: Box::new(DbNetDetector::new(self.inner.clone(), true))
                as Box<dyn Detector + Send + Sync>,
            processor: self.processor.clone(),
        }
    }
}

#[pyclass]
pub struct PyDefaultOptions {
    inner: DefaultOptions,
}

#[pymethods]
impl PyDefaultOptions {
    #[new]
    fn new(detect_size: u64, unclip_ratio: f64, text_threshold: f64, box_threshold: f64) -> Self {
        PyDefaultOptions {
            inner: DefaultOptions {
                detect_size,
                unclip_ratio,
                text_threshold,
                box_threshold,
            },
        }
    }
}

#[pyclass]
pub struct PyPreprocessorOptions {
    inner: PreprocessorOptions,
}

#[pymethods]
impl PyPreprocessorOptions {
    #[new]
    fn new(invert: bool, gamma_correct: bool, rotate: bool, auto_rotate: bool) -> Self {
        PyPreprocessorOptions {
            inner: PreprocessorOptions {
                invert,
                gamma_correct,
                rotate,
                auto_rotate,
            },
        }
    }
}

#[pyclass]
pub struct PyDetector {
    processor: Arc<Box<dyn ImageOp + Send + Sync>>,
    inner: Box<dyn Detector + Send + Sync>,
}

#[pyclass]
pub struct PyImage {
    inner: RawImage,
}

#[pymethods]
impl PyImage {
    #[new]
    fn new(path: &str) -> PyResult<Self> {
        Ok(PyImage {
            inner: RawImage::new(path).unwrap(),
        })
    }

    #[staticmethod]
    pub fn from_numpy(array: PyReadonlyArray3<u8>) -> PyResult<PyImage> {
        let dims = array.dims();
        let (height, width, channels) = (dims[0], dims[1], dims[2]);

        let array_view = array.as_array().into_iter().map(|v| *v).collect::<Vec<_>>();
        Ok(PyImage {
            inner: RawImage {
                data: array_view,
                width: width as u16,
                height: height as u16,
                channels: channels as u8,
            },
        })
    }
}

#[pymethods]
impl PyDetector {
    fn load(&mut self) -> PyResult<()> {
        self.inner.load().unwrap();
        Ok(())
    }

    fn detect(
        &mut self,
        image: PyRef<PyImage>,
        preprocessor_options: PyRef<PyPreprocessorOptions>,
        options: PyRef<PyDefaultOptions>,
    ) -> PyResult<()> {
        self.inner
            .detect(
                &image.inner,
                preprocessor_options.inner,
                options.inner.dump(),
                &*self.processor,
            )
            .unwrap();
        Ok(())
    }

    fn unload(&mut self) {
        self.inner.unload()
    }

    fn loaded(&self) -> bool {
        self.inner.loaded()
    }
}

#[pymodule]
fn rusty_manga_image_translator(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Session>()?;
    m.add_class::<PyDetector>()?;
    m.add_class::<PyImage>()?;
    m.add_class::<PyDefaultOptions>()?;
    m.add_class::<PyPreprocessorOptions>()?;
    Ok(())
}
