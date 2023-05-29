use std::fmt;
use thiserror::Error;

#[derive(Error)]
pub enum Error {
    #[error("{0}")]
    Fmt(#[from] fmt::Error),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
