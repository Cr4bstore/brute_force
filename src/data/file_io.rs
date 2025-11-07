use super::dataset::Dataset;
use crate::errors::DatasetError;
use serde_json;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

pub fn get_data_path(file_name: &str, category: &str) -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("data");
    p.push(category);
    p.push(file_name);
    p
}

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
        other => Err(DatasetError::InvalidData(other.to_string())),
    }
}

fn read_json(path: &Path) -> Result<Dataset, DatasetError> {
    // Read JSON Parse it to Dataset and return
    let text = fs::read_to_string(path).unwrap();
    // Parse String to object
    let value: Value = serde_json::from_str(&text).unwrap();

    // Next map over the value type to the dataset enum. A good idea would be to use an enum for the dataset type
    // Because then you can encapsulate the logic of mapping a Value to Dataset.

    Dataset::from_value(&value)
}
