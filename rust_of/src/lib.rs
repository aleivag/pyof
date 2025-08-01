use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod classifier;
mod attribute;
use classifier::{Classifier, Value};
use attribute::{Attribute, AttributeType};

#[derive(Serialize, Deserialize, Debug)]
struct Bucket {
    name: String,
    classifier: Classifier,
}

#[derive(Serialize, Deserialize, Debug)]
struct OfflineFeature {
    #[serde(rename = "type")]
    feature_type: String,
    python_versions: Vec<String>,
    buckets: Vec<Bucket>,
    values: HashMap<String, serde_json::Value>,
}

#[pyfunction]
fn get_bucket_name(py: Python, json_string: &str) -> PyResult<String> {
    let feature: OfflineFeature = serde_json::from_str(json_string)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;

    for bucket in &feature.buckets {
        if bucket.classifier.eval(py) {
            return Ok(bucket.name.clone());
        }
    }

    Ok("default".to_string())
}

#[pyfunction]
fn get_value_for_bucket(json_string: &str, bucket_name: &str) -> PyResult<String> {
    let feature: OfflineFeature = serde_json::from_str(json_string)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;

    if let Some(value) = feature.values.get(bucket_name) {
        Ok(value.to_string())
    } else {
        Ok("".to_string())
    }
}

#[pymodule]
fn rust_of(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_bucket_name, m)?)?;
    m.add_function(wrap_pyfunction!(get_value_for_bucket, m)?)?;
    m.add_class::<Attribute>()?;
    m.add_class::<AttributeType>()?;
    m.add_class::<Classifier>()?;
    Ok(())
}
