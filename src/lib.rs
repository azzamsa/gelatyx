#![deny(unsafe_code)]

pub mod app;
pub mod clap_app;
pub mod config;
pub mod error;
pub mod fmt;

// Aliases
pub use error::Error;
