//! The module with the error and result types of this crate
use thiserror::Error;

///
/// Error type used in this crate
///
#[derive(Error, Debug)]
pub enum LayouterError {
    #[error("IoError: {source}")]
    IoError { source: std::io::Error },
    #[error("Error from tree implementation: {source}")]
    TreeError { source: syntree::Error },
    #[error("Error occurred: {msg}")]
    OtherError { msg: String },
}

impl LayouterError {
    pub fn from_description(description: &str) -> Self {
        LayouterError::OtherError {
            msg: description.to_string(),
        }
    }
    pub fn from_io_error(io_error: std::io::Error) -> Self {
        LayouterError::IoError { source: io_error }
    }
}

///
/// Result type returned from this crate's functions
///
pub type Result<T> = std::result::Result<T, LayouterError>;
