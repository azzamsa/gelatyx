use std::path::PathBuf;

use miette::Diagnostic;
use thiserror::Error;

/// all possible errors returned by the app.
#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("{0}")]
    Internal(String),

    #[error("File is not found `{path}`.")]
    #[diagnostic(
        code(gelatyx::no_input_file),
        url(docsrs),
        help("Make sure the filename is valid.")
    )]
    FileNotFound { path: PathBuf },

    #[error("Configuration file is not found in `{path}`.")]
    #[diagnostic(
        code(gelatyx::no_config),
        url(docsrs),
        help("Try creating a config of your choosen formatter.")
    )]
    ConfigNotFound { path: PathBuf },

    #[error("Invalid configuration: {message}")]
    #[diagnostic(
        code(gelatyx::invalid_config),
        url(docsrs),
        help("See the configuration example of your choosen formatter.")
    )]
    InvalidConfig { message: String },
}

impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Internal(err.to_string())
    }
}

impl std::convert::From<&str> for Error {
    fn from(err: &str) -> Self {
        Error::Internal(err.to_string())
    }
}

impl std::convert::From<regex::Error> for Error {
    fn from(err: regex::Error) -> Self {
        Error::Internal(err.to_string())
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Internal(s)
    }
}
