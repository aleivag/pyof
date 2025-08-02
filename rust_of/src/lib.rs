use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod classifier;
mod attribute;
use classifier::{Classifier, Value};
use attribute::{Attribute, AttributeType};

#[pyclass]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FeatureType {
    #[serde(rename = "offline-feature")]
    Offline,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PythonVersion {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "py3.10")]
    Py310,
    #[serde(rename = "py3.12")]
    Py312,
    #[serde(rename = "py3.14")]
    Py314,
}

#[derive(Serialize, Deserialize, Debug)]
struct Bucket {
    name: String,
    classifier: Classifier,
}

#[pyclass]
#[derive(Serialize, Deserialize, Debug)]
struct OfflineFeature {
    #[serde(rename = "type")]
    feature_type: FeatureType,
    python_versions: Vec<PythonVersion>,
    buckets: Vec<Bucket>,
    values: HashMap<String, serde_json::Value>,
    default: Option<serde_json::Value>,
}

#[pymethods]
impl OfflineFeature {

    #[staticmethod]
    pub fn loads( py: Python, json_string: &str) -> PyResult<OfflineFeature> {
        Ok(serde_json::from_str(json_string)
        .map_err(|e| PyValueError::new_err(e.to_string()))?)

    }

fn get_bucket_name(&self, py: Python) -> PyResult<String> {

    for bucket in &self.buckets {
        if bucket.classifier.eval(py) {
            return Ok(bucket.name.clone());
        }
    }

    Ok("default".to_string())
}


    pub fn get_value_for_bucket(&self, py: Python, bucket_name: &str) -> PyResult<String> {
        // if bucket_name == "default" {
        //     // return Ok(self.default.clone().into_py(py));
        //     return Ok((&self.default).to_string());
        // }
        
        if let Some(value) = self.values.get(bucket_name) {
            Ok(value.to_string())
        } else {
            Ok("null".to_string())
        }

    }


}



#[pymodule]
fn rust_of(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Attribute>()?;
    m.add_class::<AttributeType>()?;
    m.add_class::<Classifier>()?;
    m.add_class::<OfflineFeature>()?;
    Ok(())
}
