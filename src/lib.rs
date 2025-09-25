#![cfg(any(target_os = "linux", target_os = "android"))]

use thiserror::Error;

/// define some macro_rule used in this crate.
#[macro_use]
mod macros;

pub mod _self;
pub mod pid;
pub mod proc;

/// all the funcitons return this error in the crate.
/// Any Err should be considered as an bug except `FILE_NOE_FOUND`.
#[derive(Debug, Error)]
pub enum ProcErr {
    /// Failed to read the corresponding file.
    #[error("IO error: {0}")]
    IO(std::io::Error),
    #[error("Parse error: {0:?}")]
    Parse(Box<dyn std::error::Error>),
    #[error("Bad format: {0}")]
    BadFormat(String),
}

impl From<std::io::Error> for ProcErr {
    fn from(x: std::io::Error) -> Self {
        ProcErr::IO(x)
    }
}

impl From<std::num::ParseIntError> for ProcErr {
    fn from(x: std::num::ParseIntError) -> Self {
        ProcErr::Parse(Box::new(x))
    }
}

impl From<std::num::ParseFloatError> for ProcErr {
    fn from(x: std::num::ParseFloatError) -> Self {
        ProcErr::Parse(Box::new(x))
    }
}

impl From<String> for ProcErr {
    fn from(s: String) -> Self {
        ProcErr::BadFormat(s)
    }
}

impl From<&str> for ProcErr {
    fn from(s: &str) -> Self {
        ProcErr::BadFormat(s.to_string())
    }
}
