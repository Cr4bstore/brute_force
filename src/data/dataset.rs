use crate::errors::DatasetError;
use serde_json::Value;
use std::collections::BTreeMap;

// ---------- Helpers ----------
fn type_name(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Number(n) => {
            if n.as_i64().is_some() {
                "i64"
            } else if n.as_u64().is_some() {
                "u64"
            } else {
                "f64"
            }
        }
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

fn find_start_index(arr: &[Value]) -> usize {
    arr.iter().position(|v| !v.is_null()).unwrap_or(arr.len())
}

// ---------- Core data types ----------
#[derive(Debug, Clone)]
pub enum Series {
    F64 { starting_index: usize, values: Vec<f64> },
    I64 { starting_index: usize, values: Vec<i64> },
    Bool { starting_index: usize, values: Vec<bool> },
    Str { starting_index: usize, values: Vec<String> },
}

#[derive(Debug, Clone, Default)]
pub struct Dataset {
    pub columns: BTreeMap<String, Series>,
    pub len: usize,
}

// ---------- Implementation ----------
impl Dataset {
    pub fn from_value(value: &Value) -> Result<Self, DatasetError> {
        let src = value.as_object().ok_or_else(|| {
            DatasetError::InvalidFormat("Expected JSON object at top level".to_string())
        })?;

        let mut ds = Dataset::default();

        if let Some(time_val) = src.get("time") {
            println!("Time column type: {}", type_name(time_val));
        }

        if let Some(lt_blue_wave) = src.get("Lt Blue Wave").and_then(|v| v.as_array()) {
            for (i, value) in lt_blue_wave.iter().enumerate() {
                println!("Index: {}, Value: {}, Type: {}", i, value, type_name(value));
                if i > 200 {
                    println!("... (remaining elements omitted)");
                    break;
                }
            }
        }

        // ---- Parse all columns ----
        for (key, val) in src.iter() {
            if let Some(arr) = val.as_array() {
                let start_index = find_start_index(arr);
                let first_non_null = arr.iter().find(|v| !v.is_null());

                let series = match first_non_null {
                    Some(Value::Number(_)) => {
                        let values: Vec<f64> = arr.iter()
                            .skip(start_index)
                            .filter_map(|v| v.as_f64())
                            .collect();
                        Series::F64 { starting_index: start_index, values }
                    }
                    Some(Value::Bool(_)) => {
                        let values: Vec<bool> = arr.iter()
                            .skip(start_index)
                            .filter_map(|v| v.as_bool())
                            .collect();
                        Series::Bool { starting_index: start_index, values }
                    }
                    Some(Value::String(_)) => {
                        let values: Vec<String> = arr.iter()
                            .skip(start_index)
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect();
                        Series::Str { starting_index: start_index, values }
                    }
                    _ => Series::F64 { starting_index: arr.len(), values: vec![] },
                };

                ds.columns.insert(key.clone(), series);
            }
        }

        // ---- Compute dataset length ----
        ds.len = ds
            .columns
            .values()
            .map(|s| match s {
                Series::F64 { starting_index, values } => starting_index + values.len(),
                Series::I64 { starting_index, values } => starting_index + values.len(),
                Series::Bool { starting_index, values } => starting_index + values.len(),
                Series::Str { starting_index, values } => starting_index + values.len(),
            })
            .max()
            .unwrap_or(0);

        Ok(ds)
    }
}