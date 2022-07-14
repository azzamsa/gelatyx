use crate::Error;
use std::fs;
use std::path::Path;

pub fn read_file(path: &Path) -> Result<String, Error> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(err) => Err(Error::FileUnreadable(err.to_string())),
    }
}

pub fn write_file(path: &Path, content: String) -> Result<(), Error> {
    match fs::write(path, content) {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::FileUnwritable(err.to_string())),
    }
}
