use std::{error::Error, fmt::Display};

pub type OResult<T> = Result<T, Box<dyn Error>>;

/// If the model is not found on the official ollama website.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ModelNotFound(pub String);

impl Display for ModelNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ModelNotFound {}
