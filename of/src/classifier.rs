use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyList;
use serde::{Deserialize, Serialize};

use crate::attribute::Attribute;

use regex::Regex;

// Represents the flexible `value` field in a Classifier.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ClassifierValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Classifier(Box<Classifier>),
    Array(Vec<ClassifierValue>),
}

impl<'source> FromPyObject<'source> for ClassifierValue {
    fn extract(ob: &PyAny) -> PyResult<Self> {
        if let Ok(s) = ob.extract::<String>() {
            Ok(ClassifierValue::String(s))
        } else if let Ok(f) = ob.extract::<f64>() {
            Ok(ClassifierValue::Number(f))
        } else if let Ok(b) = ob.extract::<bool>() {
            Ok(ClassifierValue::Boolean(b))
        } else if let Ok(c) = ob.extract::<PyRef<Classifier>>() {
            // Extract the underlying Rust Classifier from the PyRef
            Ok(ClassifierValue::Classifier(Box::new(c.clone())))
        } else if let Ok(l) = ob.extract::<Py<PyList>>() {
            let mut vec = Vec::new();
            for item in l.as_ref(ob.py()).iter() {
                vec.push(item.extract()?);
            }
            Ok(ClassifierValue::Array(vec))
        } else {
            Err(PyValueError::new_err(
                "Could not convert Python object to ClassifierValue",
            ))
        }
    }
}

impl IntoPy<PyObject> for ClassifierValue {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            ClassifierValue::String(s) => s.into_py(py),
            ClassifierValue::Number(f) => f.into_py(py),
            ClassifierValue::Boolean(b) => b.into_py(py),
            ClassifierValue::Classifier(c) => c.into_py(py),
            ClassifierValue::Array(v) => v.into_py(py),
        }
    }
}

TypeEnum!(
    Classifier=>bool,
    "re.match" => REGEXMATCH { attribute: Attribute, value: ClassifierValue } => {
        |py:Python| {
            if let (ClassifierValue::String(pattern), Ok(attr_py_obj)) = (value, attribute.eval(py)) {
            if let Ok(hostname) = attr_py_obj.extract::<String>(py) {
                let re = Regex::new(pattern).unwrap();
                re.is_match(&hostname)
            } else {
                false
            }
        } else {
            false
        }
        }
    },
    "bool.all" => ALL { value: Vec<Classifier> } =>  { |py:Python| {  value.iter().all(|c| c.eval(py)) }
    },
    "bool.any" => ANY { value: Vec<Classifier> } => { |py: Python| { value.iter().any(|c| c.eval(py)) }
    },
    "comparison.lt" => LT { attribute: Attribute, value: ClassifierValue } => {
        |py:Python| {
        if let (ClassifierValue::Number(val), Ok(attr_py_obj)) = (value, attribute.eval(py)) {
            if let Ok(attr_val) = attr_py_obj.extract::<f64>(py) {
                attr_val < *val
            } else {
                false
            }
        } else {
            false
        }
    }
    },
    "comparison.gt" => GT { attribute: Attribute, value: ClassifierValue } => {

        |py:Python| {
        if let (ClassifierValue::Number(val), Ok(attr_py_obj)) = (value, attribute.eval(py)) {
            if let Ok(attr_val) = attr_py_obj.extract::<f64>(py) {
                attr_val > *val
            } else {
                false
            }
        } else {
            false
        }}
    },
    "comparison.gte" => GTE { attribute: Attribute, value: ClassifierValue } => {
        |py:Python| {
        if let (ClassifierValue::Number(val), Ok(attr_py_obj)) = (value, attribute.eval(py)) {
            if let Ok(attr_val) = attr_py_obj.extract::<f64>(py) {
                attr_val >= *val
            } else {
                false
            }
        } else {
            false
        }}
    },
    "comparison.lte" => LTE { attribute: Attribute, value: ClassifierValue } => {
        |py:Python| {
        if let (ClassifierValue::Number(val), Ok(attr_py_obj)) = (value, attribute.eval(py)) {
            if let Ok(attr_val) = attr_py_obj.extract::<f64>(py) {
                attr_val <= *val
            } else {
                false
            }
        } else {
            false
        }}
    },
    "comparison.eq" => EQ { attribute: Attribute, value: ClassifierValue } => {
        |py:Python| {
        let attr_py_obj_res = attribute.eval(py);
        if let Ok(attr_py_obj) = attr_py_obj_res {
            if let ClassifierValue::Number(val) = value {
                if let Ok(attr_val) = attr_py_obj.extract::<f64>(py) {
                    attr_val == *val
                } else {
                    false
                }
            } else if let ClassifierValue::String(val) = value {
                if let Ok(attr_val) = attr_py_obj.extract::<String>(py) {
                    attr_val == *val
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }}
    }
);
