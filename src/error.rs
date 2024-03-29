use std::{fmt, io};

use simplicity::elements;
use thiserror::Error;

#[derive(Error)]
pub enum Error {
    #[error("{0}")]
    Fmt(#[from] fmt::Error),
    #[error("{0}")]
    IO(#[from] io::Error),
    // #[error("{0}")]
    // Hex(#[from] elements::hex::Error),
    #[error("{0}")]
    Encode(#[from] elements::encode::Error),
    #[error("{0}")]
    Json(#[from] serde_json::Error),
    #[error("{0}")]
    Base64(#[from] base64::DecodeError),
    #[error("{0}")]
    Simplicity(#[from] simplicity::Error),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
