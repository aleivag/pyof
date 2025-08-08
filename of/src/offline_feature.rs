use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::classifier::Classifier;
use crate::values::FeatureValue;
use crate::type_enum::{FeatureType, PythonVersion};

#[pyclass]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bucket {
    name: String,
    classifier: Classifier,
    value: FeatureValue,
}

#[pymethods]
impl Bucket {
    #[new]
    fn new(name: String, classifier: Classifier, value: FeatureValue) -> PyResult<Self> {
        if name == "default" {
            return Err(PyValueError::new_err(
                "Cannot name a bucket 'default'. 'default' is a reserved name.",
            ));
        }
        Ok(Bucket {
            name,
            classifier,
            value,
        })
    }
}

#[pyclass]
#[derive(Serialize, Deserialize, Debug)]
pub struct OfflineFeature {
    #[serde(rename = "type")]
    feature_type: FeatureType,
    python_versions: Vec<PythonVersion>,
    buckets: Vec<Bucket>,
    default: FeatureValue,
}

#[pymethods]
impl OfflineFeature {
    #[new]
    fn new(
        feature_type: FeatureType,
        python_versions: Vec<PythonVersion>,
        buckets: Vec<Bucket>,
        default: FeatureValue,
    ) -> PyResult<Self> {
        Ok(OfflineFeature {
            feature_type,
            python_versions,
            buckets,
            default,
        })
    }

    #[staticmethod]
    pub fn loads(py: Python, json_string: &str) -> PyResult<OfflineFeature> {
        Ok(serde_json::from_str(json_string).map_err(|e| PyValueError::new_err(e.to_string()))?)
    }

    pub fn dumps(&self, py: Python, indent: Option<bool>) -> String {
        if indent.unwrap_or(false) {
            serde_json::to_string_pretty(&self).unwrap()
        } else {
            serde_json::to_string(&self).unwrap()
        }
    }
    
    #[pyo3(signature = (path, only_if_changed = true))]
    pub fn write_to_disk(&self, py: Python, path: String, only_if_changed: bool) -> PyResult<()> {
        use std::fs;
        use std::io::Write;

        let new_content = self.dumps(py, Some(true)); 

        if only_if_changed {
            if let Ok(existing_content) = fs::read_to_string(&path) {
                if existing_content == new_content {
                    return Ok(()); // No change, so do nothing
                }
            }
        }

        let mut file = fs::File::create(&path)
            .map_err(|e| PyValueError::new_err(format!("Failed to create file {}: {}", path, e)))?;
        file.write_all(new_content.as_bytes())
            .map_err(|e| PyValueError::new_err(format!("Failed to write to file {}: {}", path, e)))?;

        Ok(())
    }
    

    fn get_bucket(&self, py: Python) -> PyResult<Option<Bucket>> {
        for bucket in &self.buckets {
            match bucket.classifier.eval(py) {
                Ok(true) => return Ok(Some(bucket.clone())),
                Ok(false) => continue,
                Err(_) => continue, // Treat errors as a non-match
            }
        }
        Ok(None)
    }


    fn get_bucket_name(&self, py: Python) -> PyResult<String> {
        match self.get_bucket(py)? {
            Some(bucket) => Ok(bucket.name),
            None =>  Ok("default".to_string()),
        } 
    }

    pub fn get_bucket_name_and_value(&self, py: Python) -> PyResult<(String, FeatureValue)> {
        match  self.get_bucket(py)? {
            Some(bucket) =>         Ok((bucket.name.to_string(), bucket.value.clone())),
            None => Ok(("default".to_string(), (&self.default).clone()))
        }
    }
}