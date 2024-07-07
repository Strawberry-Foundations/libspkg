use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum BinPkgError {
    InvalidFormat,
    SourcePathNotFound
}

impl Display for BinPkgError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BinPkgError::InvalidFormat => write!(f, "The specified package is not compatible with the binpkg format."),
            BinPkgError::SourcePathNotFound => write!(f, "Couldn't find source path for binpkg"),
        }
    }
}

impl Error for BinPkgError {}
