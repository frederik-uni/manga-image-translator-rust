use std::sync::Arc;

use base_util::onnx::{all_providers, Providers};
use dbnet::{DbNetDetector, DefaultOptions};
use interface::{
    detectors::{textlines::Quadrilateral, Detector, PreprocessorOptions},
    image::{CpuImageProcessor, ImageOp, RawImage},
    model::{CreateData, Model as _},
};
use numpy::{
    ndarray::{Array2, Array3},
    IntoPyArray as _, PyArray2, PyArray3, PyArrayMethods, PyReadonlyArray3,
};
use parking_lot::Mutex;
use pyo3::{exceptions::PyRuntimeError, prelude::*};

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
            inner: Arc::new(Mutex::new(
                Box::new(DbNetDetector::new(self.inner.clone(), false))
                    as Box<dyn Detector + Send + Sync>,
            )),
            processor: self.processor.clone(),
        }
    }

    fn convnext_detector(&self) -> PyDetector {
        PyDetector {
            inner: Arc::new(Mutex::new(
                Box::new(DbNetDetector::new(self.inner.clone(), true))
                    as Box<dyn Detector + Send + Sync>,
            )),
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
    inner: Arc<Mutex<Box<dyn Detector + Send + Sync>>>,
}

#[pyclass]
pub struct PyImage {
    inner: Arc<RawImage>,
}

#[pymethods]
impl PyImage {
    #[new]
    fn new(path: &str) -> PyResult<Self> {
        let v = RawImage::new(path).map_err(|v| PyRuntimeError::new_err(v.to_string()))?;

        Ok(PyImage { inner: Arc::new(v) })
    }

    pub fn to_numpy<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyArray3<u8>>> {
        let data = self.inner.data.clone();
        let (width, height, channels) = (
            self.inner.width as usize,
            self.inner.height as usize,
            self.inner.channels as usize,
        );
        let array = Array3::from_shape_vec((height, width, channels), data)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        Ok(array.into_pyarray(py))
    }

    #[staticmethod]
    pub fn from_numpy(array: PyReadonlyArray3<u8>) -> PyResult<PyImage> {
        let dims = array.dims();
        let (height, width, channels) = (dims[0], dims[1], dims[2]);

        let array_view = array.as_array().into_iter().map(|v| *v).collect::<Vec<_>>();
        Ok(PyImage {
            inner: Arc::new(RawImage {
                data: array_view,
                width: width as u16,
                height: height as u16,
                channels: channels as u8,
            }),
        })
    }
}

#[pyclass]
pub struct PyQuadrilateral {
    inner: Quadrilateral,
}

#[pymethods]
impl PyQuadrilateral {
    fn score(&self) -> f64 {
        self.inner.score()
    }

    fn aspect_ratio(&self) -> f64 {
        self.inner.aspect_ratio()
    }

    fn area(&self) -> f64 {
        self.inner.area()
    }

    fn vertical(&self) -> bool {
        self.inner.vertical()
    }

    fn pts(&self) -> Vec<(i64, i64)> {
        self.inner.pts().to_vec()
    }

    fn structure(&self) -> Vec<(i64, i64)> {
        self.inner.structure().to_vec()
    }
}

#[pymethods]
impl PyDetector {
    fn load(&self) -> PyResult<()> {
        self.inner
            .lock()
            .load()
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }

    fn detect<'py>(
        &mut self,
        py: Python<'py>,
        image: PyRef<PyImage>,
        preprocessor_options: PyRef<PyPreprocessorOptions>,
        options: PyRef<PyDefaultOptions>,
    ) -> PyResult<(Vec<PyQuadrilateral>, Bound<'py, PyArray2<u8>>)> {
        let inner = self.inner.clone();
        let preprocessor_options = preprocessor_options.inner;
        let options = options.inner;
        let img = image.inner.clone();
        let processor = self.processor.clone();
        let det = py
            .allow_threads(|| {
                inner
                    .lock()
                    .detect(&img, preprocessor_options, options.dump(), &*processor)
            })
            .map_err(|e| PyRuntimeError::new_err(e.to_string()));
        let (qua, mask) = det?;
        let data = mask.data.clone();
        let (width, height) = (mask.width as usize, mask.height as usize);
        let array = Array2::from_shape_vec((height, width), data)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;

        let qua = qua
            .into_iter()
            .map(|v| PyQuadrilateral { inner: v })
            .collect();
        Ok((qua, array.into_pyarray(py)))
    }

    fn unload(&mut self) {
        self.inner.lock().unload()
    }

    fn loaded(&self) -> bool {
        self.inner.lock().loaded()
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
