use super::dataset::Dataset;
use crate::errors::DatasetError;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

/// Returns an absolute path to a dataset file within the `data/<category>` directory.
pub fn get_data_path(file_name: &str, category: &str) -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("data");
    p.push(category);
    p.push(file_name);
    p
}

/// Reads a dataset file and automatically determines its format by extension.
///
/// # Errors
/// Returns a `DatasetError` if the file cannot be read, parsed, or if the extension is unsupported.
pub fn read_file<P: AsRef<Path>>(path: P) -> Result<Dataset, DatasetError> {
    let path = path.as_ref();
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_ascii_lowercase())
        .unwrap_or_default();

    match ext.as_str() {
        "json" => read_json(path),
        // "csv" => read_csv(path),
        other => Err(DatasetError::UnsupportedExtension(other.to_string())),
    }
}

/// Reads and parses a `.json` dataset file into a [`Dataset`].
///
/// # Errors
/// - Returns `DatasetError::IoError` if the file cannot be opened.
/// - Returns `DatasetError::InvalidFormat` if JSON parsing fails.
/// - Returns `DatasetError::InvalidStructure` if top-level JSON is not an object.
fn read_json(path: &Path) -> Result<Dataset, DatasetError> {
    // Read JSON file as text
    let text = fs::read_to_string(path)
        .map_err(|e| DatasetError::IoError(e.to_string()))?;

    // Parse string into serde_json::Value
    let value: Value = serde_json::from_str(&text)
        .map_err(DatasetError::Json)?;

    // Convert Value -> Dataset
    Dataset::from_value(&value)
}

/// Reads and parses a `.csv` dataset file into a [`Dataset`].
///
/// # TODO
/// Implement CSV parsing logic using the `csv` crate:
/// - Read headers as column names
/// - Parse each column into a suitable [`Series`] type
/// - Determine `starting_index` automatically if nullish prefixes are used
fn read_csv(_path: &Path) -> Result<Dataset, DatasetError> {
    Err(DatasetError::UnsupportedExtension(
        "CSV parsing not implemented yet".to_string(),
    ))
}