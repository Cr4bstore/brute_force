use std::fmt;
use serde_json;

/// All possible errors that can occur when reading or parsing a dataset file.
#[derive(Debug)]
pub enum DatasetError {
    /// File system error (e.g., cannot open or read file)
    IoError(String),

    /// Data structure or parsing issue (invalid JSON, wrong format, etc.)
    InvalidFormat(String),

    /// When a file extension is unsupported (e.g., `.txt`, `.yaml`)
    UnsupportedExtension(String),

    /// When the data inside is inconsistent, missing keys, or invalid
    InvalidData(String),

    /// When the data structure does not match expected schema
    InvalidStructure(String),

    /// When JSON parsing fails
    Json(serde_json::Error),
}

// Implement Display + Error for good ergonomics (`?` works automatically)
impl fmt::Display for DatasetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatasetError::IoError(msg) => write!(f, "I/O error: {}", msg),
            DatasetError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            DatasetError::UnsupportedExtension(ext) => write!(f, "Unsupported file extension: {}", ext),
            DatasetError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            DatasetError::InvalidStructure(msg) => write!(f, "Invalid structure: {}", msg),
            DatasetError::Json(err) => write!(f, "JSON parsing error: {}", err),
        }
    }
}

impl std::error::Error for DatasetError {}