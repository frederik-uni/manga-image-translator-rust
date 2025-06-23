use std::{collections::HashMap, sync::Arc};

use base_util::onnx::{all_providers, Providers};

use crate::models::ModelDb;

pub trait Model {
    fn name(&self) -> &'static str;
    fn kind(&self) -> &'static str;
    fn models(&self) -> HashMap<&'static str, ModelSource>;
    fn loaded(&self) -> bool;
    fn unload(&mut self);
    fn load(&mut self) -> anyhow::Result<()>;
}

#[derive(Clone)]
pub struct CreateData {
    pub mode_db: Arc<ModelDb>,
    pub providers: Vec<Providers>,
}

impl CreateData {
    pub fn all() -> Self {
        Self {
            mode_db: Arc::new(ModelDb {}),
            providers: all_providers(),
        }
    }

    pub fn new(providers: Vec<Providers>) -> Self {
        Self {
            mode_db: Arc::new(ModelDb {}),
            providers,
        }
    }
}

pub struct ModelSource {
    pub url: &'static str,
    pub hash: &'static str,
    pub archive: Option<HashMap<&'static str, &'static str>>,
    pub file: Option<&'static str>,
}
