use std::path::PathBuf;
use thiserror::Error;

/// all possible errors returned by the app.
#[derive(Error, Debug)]
pub enum Error {
    #[error("{0:?}")]
    Internal(String),

    #[error("No such file {0:?}")]
    FileNotFound(PathBuf),

    #[error("Failed to read the file {0:?}")]
    FileUnreadable(String),

    #[error("Failed to write the file {0:?}")]
    FileUnwritable(String),
}

impl std::convert::From<regex::Error> for Error {
    fn from(err: regex::Error) -> Self {
        Error::Internal(err.to_string())
    }
}

impl std::convert::From<&str> for Error {
    fn from(err: &str) -> Self {
        Error::Internal(err.to_string())
    }
}

impl std::convert::From<stylua_lib::Error> for Error {
    fn from(err: stylua_lib::Error) -> Self {
        Error::Internal(err.to_string())
    }
}
