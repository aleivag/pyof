use crate::classifier::ClassifierValue;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyFloat, PyList, PyString};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Represents the flexible `value` field in a Classifier.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum FeatureValue {
    Boolean(bool),
    String(String),
    Number(f64),
    Array(Vec<FeatureValue>),
    Map(HashMap<String, FeatureValue>),
    Null,
}

impl<'source> FromPyObject<'source> for FeatureValue {
    fn extract(ob: &PyAny) -> PyResult<Self> {
        if let Ok(s) = ob.extract::<String>() {
            Ok(FeatureValue::String(s))
        } 
        else if let Ok(b) = ob.extract::<bool>() {
            Ok(FeatureValue::Boolean(b))
        }
        else if let Ok(f) = ob.extract::<f64>() {
            Ok(FeatureValue::Number(f))
        } else if let Ok(b) = ob.extract::<bool>() {
            Ok(FeatureValue::Boolean(b))
        } else if let Ok(l) = ob.extract::<Py<PyList>>() {
            let mut vec = Vec::new();
            for item in l.as_ref(ob.py()).iter() {
                vec.push(item.extract()?);
            }
            Ok(FeatureValue::Array(vec))
        } else if let Ok(d) = ob.downcast::<PyDict>() {
            let mut map = HashMap::new();
            for (key, value) in d.iter() {
                map.insert(key.extract()?, value.extract()?);
            }
            Ok(FeatureValue::Map(map))
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
            FeatureValue::Map(m) => m.into_py(py),
            FeatureValue::Null => py.None(),
        }
    }
}
