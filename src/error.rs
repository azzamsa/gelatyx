use thiserror::Error;

/// all possible errors returned by the app.
#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Internal(String),

    #[error("{0}")]
    InvalidArgument(String),
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
