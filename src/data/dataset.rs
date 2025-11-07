use crate::errors::DatasetError;
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub enum Series {
    F64(Vec<f64>),
    I64(Vec<i64>),
    Bool(Vec<bool>),
    Str(Vec<String>),
}

#[derive(Debug, Clone, Default)]
pub struct Dataset {
    pub columns: BTreeMap<String, Series>,
    pub len: usize,
}

impl Dataset {
    pub fn from_value(value: &Value) -> Result<Self, DatasetError> {
        let obj = value;

        // Now parse the data into Dataset
        let ds = parse_into_dataset(obj)?;

        // Map the values

        Ok(ds)
    }
}

fn parse_into_dataset(obj: &Value) -> Result<Dataset, DatasetError> {
    // Do something here

    println!("{}", obj);

    if false {
        return Err(DatasetError::InvalidData(
            "Something went wrong".to_string(),
        ));
    }

    Ok(Dataset::default())
}
