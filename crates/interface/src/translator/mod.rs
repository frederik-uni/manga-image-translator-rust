use std::{collections::HashMap, future::Future};

pub trait Translator {
    type Options;
    fn is_sync(&self) -> bool;
    fn translate_async(
        &self,
        text: Vec<String>,
        options: Self::Options,
    ) -> dyn Future<Output = TranslatorChain>;
    fn translate(&self, text: Vec<String>, options: Self::Options) -> TranslatorChain;
}

pub struct TranslatorChain {
    order: Vec<TranslationKind>,
    tranlations: HashMap<TranslationKind, Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TranslationKind {
    translator: String,
    language: String,
}
