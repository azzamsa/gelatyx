use ansi_term::Colour::Red;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

/// all possible errors returned by the app.
#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] ::std::io::Error),

    #[error("{0}")]
    Msg(String),
}

impl std::convert::From<regex::Error> for Error {
    fn from(err: regex::Error) -> Self {
        Error::Msg(err.to_string())
    }
}

impl std::convert::From<&str> for Error {
    fn from(err: &str) -> Self {
        Error::Msg(err.to_string())
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Msg(s)
    }
}
impl std::convert::From<stylua_lib::Error> for Error {
    fn from(error: stylua_lib::Error) -> Self {
        Error::Msg(format!("stylua: {}", error))
    }
}

pub fn default_error_handler(error: &Error) {
    match error {
        Error::Io(ref io_error) if io_error.kind() == ::std::io::ErrorKind::BrokenPipe => {
            ::std::process::exit(0);
        }
        _ => {
            eprintln!("{}: {}", Red.paint("[gelatyx error]"), error);
        }
    };
}
