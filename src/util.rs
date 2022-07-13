use crate::Error;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub fn read_file(path: PathBuf) -> Result<String, Error> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(err) => Err(Error::FileUnreadable(err.to_string())),
    }
}

pub fn write_file(path: PathBuf, content: String) -> Result<(), Error> {
    match fs::write(path, content) {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::FileUnwritable(err.to_string())),
    }
}
pub fn is_exist(path: &str) -> Result<PathBuf, Error> {
    let path = Path::new(path);

    if path.exists() {
        Ok(path.to_path_buf())
    } else {
        Err(Error::FileNotFound(path.to_path_buf()))
    }
}
