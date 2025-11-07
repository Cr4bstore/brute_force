#[derive(Debug)]
pub enum DatasetError {
    InvalidStructure(String),
    InvalidData(String),
    Io(std::io::Error),
    Json(serde_json::Error),
}
