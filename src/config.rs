use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Format,
    Check,
}

#[derive(Debug, Clone)]
pub struct Config<'a> {
    /// The language to use
    pub language: &'a str,

    /// List of files to print
    pub files: Vec<&'a Path>,

    /// Whether or not the output should be colorized
    pub colored_output: bool,

    /// Format the docs or check only
    pub mode: Mode,
}
