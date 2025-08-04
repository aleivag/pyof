use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[macro_use]
mod type_enum;
mod attribute;
mod classifier;
mod values;
use attribute::Attribute;
use classifier::Classifier;
use values::FeatureValue;

macro_rules! PythonEnum {
    (
        $enum_name:ident, // The name of the enum (e.g., Classifier)
        $($serde_name:literal => $variant_name:ident),*
    ) => {
        #[pyclass]
        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub enum $enum_name {
            $(
                #[serde(rename = $serde_name)]
                $variant_name,
            )*
        }

}}

PythonEnum! {
    FeatureType,
    "offline-feature" => Offline
}

PythonEnum! {
    PythonVersion,
    "all" => All,
    "py3.10" => Py310,
    "py3.12" => Py312,
    "py3.14" => Py314
}

#[pyclass]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Bucket {
    name: String,
    classifier: Classifier,
}

#[pymethods]
impl Bucket {
    #[new]
    fn new(name: String, classifier: Classifier) -> Self {
        Bucket { name, classifier }
    }
}

#[pyclass]
#[derive(Serialize, Deserialize, Debug)]
struct OfflineFeature {
    #[serde(rename = "type")]
    feature_type: FeatureType,
    python_versions: Vec<PythonVersion>,
    buckets: Vec<Bucket>,
    values: HashMap<String, FeatureValue>,
    default: FeatureValue,
}

#[pymethods]
impl OfflineFeature {
    #[new]
    fn new(
        feature_type: FeatureType,
        python_versions: Vec<PythonVersion>,
        buckets: Vec<Bucket>,
        values: HashMap<String, FeatureValue>,
        default: FeatureValue,
    ) -> PyResult<Self> {
        if values.contains_key("default") {
            return Err(PyValueError::new_err(
                "'default' key is not allowed in the 'values' HashMap.",
            ));
        }

        Ok(OfflineFeature {
            feature_type,
            python_versions,
            buckets,
            values,
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

    fn get_bucket_name(&self, py: Python) -> PyResult<String> {
        for bucket in &self.buckets {
            if bucket.classifier.eval(py) {
                return Ok(bucket.name.clone());
            }
        }

        Ok("default".to_string())
    }

    pub fn get_value_for_bucket(&self, bucket_name: &str) -> PyResult<FeatureValue> {
        if bucket_name == "default" {
            return Ok((&self.default).clone());
        }

        let value = self.values.get(bucket_name).unwrap();
        Ok(value.clone())
    }

    pub fn get_bucket_and_value(&self, py: Python) -> PyResult<(String, FeatureValue)> {
        let name = &self.get_bucket_name(py)?;
        let value = &self.get_value_for_bucket(name)?;
        Ok((name.to_string(), value.clone()))
    }
}

#[pymodule]
fn of(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Bucket>()?;
    m.add_class::<FeatureType>()?;
    m.add_class::<PythonVersion>()?;
    m.add_class::<Attribute>()?;
    m.add_class::<Classifier>()?;
    m.add_class::<OfflineFeature>()?;
    Ok(())
}
