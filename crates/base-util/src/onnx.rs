use std::path::PathBuf;

use ndarray::{Array, Array2, IxDyn};
use ort::{
    execution_providers::{
        CUDAExecutionProvider, CoreMLExecutionProvider, DirectMLExecutionProvider,
        TensorRTExecutionProvider,
    },
    session::{
        builder::{GraphOptimizationLevel, SessionBuilder},
        Session,
    },
};

use crate::error::ModelLoadError;

#[derive(Clone)]
pub enum Providers {
    TensorRT,
    CUDA,
    DirectML,
    CoreML,
}

pub fn all_providers() -> Vec<Providers> {
    vec![
        #[cfg(any(target_os = "windows", target_os = "linux"))]
        Providers::TensorRT,
        #[cfg(any(target_os = "windows", target_os = "linux"))]
        Providers::CUDA,
        #[cfg(target_os = "windows")]
        Providers::DirectML,
        #[cfg(target_os = "macos")]
        Providers::CoreML,
    ]
}

pub fn new_session(path: PathBuf, providers: Vec<Providers>) -> Result<Session, ModelLoadError> {
    Ok(new_session_(Session::builder()?, providers)?.commit_from_file(path)?)
}

pub fn new_session_(
    session_builder: SessionBuilder,
    providers: Vec<Providers>,
) -> Result<SessionBuilder, ort::Error> {
    let providers = providers
        .into_iter()
        .map(|v| match v {
            Providers::TensorRT => TensorRTExecutionProvider::default().build(),
            Providers::CUDA => CUDAExecutionProvider::default().build(),
            Providers::DirectML => DirectMLExecutionProvider::default().build(),
            Providers::CoreML => CoreMLExecutionProvider::default().build(),
        })
        .collect::<Vec<_>>();
    Ok(session_builder
        .with_optimization_level(GraphOptimizationLevel::Level3)?
        .with_execution_providers(providers)?
        .with_parallel_execution(true)?
        .with_intra_threads(4)?
        .with_inter_threads(2)?)
}

pub fn dyn_to_2d(arr: Array<f32, IxDyn>) -> Option<Array2<f32>> {
    if arr.ndim() == 2 {
        let shape = arr.shape();
        let (rows, cols) = (shape[0], shape[1]);

        arr.into_shape((rows, cols)).ok()
    } else {
        None
    }
}
