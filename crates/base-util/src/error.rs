#[derive(thiserror::Error, Debug)]
pub enum Error {
    ModelCreate(#[from] ModelLoadError),
    Preprocess(#[from] PreProcessingError),
    Processing(#[from] ProcessingError),
    Postprocess(#[from] PostProcessingError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ModelLoadError {
    CouldntCreateFile(#[from] std::io::Error),
    DownloadFailed(#[from] ureq::Error),
    CreateSession(#[from] ort::Error),
    ModelNotRegistered,
}

impl std::fmt::Display for ModelLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::fmt::Display for PreProcessingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::fmt::Display for PostProcessingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
#[derive(thiserror::Error, Debug)]
pub enum ProcessingError {
    Model(#[from] ort::Error),
    Extract(#[from] ndarray::ShapeError),
}

#[derive(thiserror::Error, Debug)]
pub enum PreProcessingError {
    OpenCv(#[from] opencv::Error),
    NdArray(#[from] ndarray::ShapeError),
    Empty,
}

#[derive(thiserror::Error, Debug)]
pub enum PostProcessingError {
    OpenCv(#[from] opencv::Error),
    Empty,
    NdArray(#[from] ndarray::ShapeError),
}
