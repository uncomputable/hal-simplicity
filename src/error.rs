use simplicity::bitcoin_hashes;
use std::{fmt, io};
use thiserror::Error;

#[derive(Error)]
pub enum Error {
    #[error("{0}")]
    Fmt(#[from] fmt::Error),
    #[error("{0}")]
    IO(#[from] io::Error),
    #[error("{0}")]
    Hex(#[from] bitcoin_hashes::hex::Error),
    #[error("{0}")]
    Encode(#[from] elements::encode::Error),
    #[error("{0}")]
    Json(#[from] serde_json::Error),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
