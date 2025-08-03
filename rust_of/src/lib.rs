use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use pyo3::types::{PyBool, PyDict, PyFloat, PyList, PyString};
mod attribute;
mod classifier;
use attribute::{Attribute, AttributeType};
use classifier::{Classifier, Value};

// Represents the flexible `value` field in a Classifier.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum FeatureValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<Value>),
    Null,
}

impl<'source> FromPyObject<'source> for FeatureValue {
    fn extract(ob: &PyAny) -> PyResult<Self> {
        if let Ok(s) = ob.extract::<String>() {
            Ok(FeatureValue::String(s))
        } else if let Ok(f) = ob.extract::<f64>() {
            Ok(FeatureValue::Number(f))
        } else if let Ok(b) = ob.extract::<bool>() {
            Ok(FeatureValue::Boolean(b))
        } else if let Ok(l) = ob.extract::<Py<PyList>>() {
            let mut vec = Vec::new();
            for item in l.as_ref(ob.py()).iter() {
                vec.push(item.extract()?);
            }
            Ok(FeatureValue::Array(vec))
        } else if ob.is_none() {
            Ok(FeatureValue::Null)
        } else {
            Err(PyValueError::new_err(
                "Could not convert Python object to Value",
            ))
        }
    }
}

impl IntoPy<PyObject> for FeatureValue {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            FeatureValue::String(s) => s.into_py(py),
            FeatureValue::Number(f) => f.into_py(py),
            FeatureValue::Boolean(b) => b.into_py(py),
            FeatureValue::Array(v) => v.into_py(py),
            FeatureValue::Null => py.None(),
        }
    }
}
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

    pub fn dumps(&self, py: Python) -> String {
        serde_json::to_string(&self).unwrap()
    }

    fn get_bucket_name(&self, py: Python) -> PyResult<String> {
        for bucket in &self.buckets {
            if bucket.classifier.eval(py) {
                return Ok(bucket.name.clone());
            }
        }

        Ok("default".to_string())
    }

    pub fn get_value_for_bucket(&self, py: Python, bucket_name: &str) -> PyResult<PyObject> {
        if bucket_name == "default" {
            return Ok((&self.default).clone().into_py(py));
        }

        if let Some(value) = self.values.get(bucket_name) {
            Ok(value.clone().into_py(py))
        } else {
            Ok(py.None())
        }
    }
}

#[pymodule]
fn rust_of(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Bucket>()?;
    m.add_class::<FeatureType>()?;
    m.add_class::<PythonVersion>()?;
    m.add_class::<Attribute>()?;
    m.add_class::<AttributeType>()?;
    m.add_class::<Classifier>()?;
    m.add_class::<OfflineFeature>()?;
    Ok(())
}
