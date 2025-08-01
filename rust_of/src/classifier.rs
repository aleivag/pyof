use serde::{Deserialize, Serialize};
use pyo3::prelude::*;
use pyo3::types::{PyString, PyFloat, PyBool, PyList, PyDict};
use pyo3::exceptions::PyValueError;

use crate::attribute::{Attribute};

use std::collections::HashMap;
use regex::Regex;


// Represents the flexible `value` field in a Classifier.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Classifier(Box<Classifier>),
    Array(Vec<Value>),
}

impl<'source> FromPyObject<'source> for Value {
    fn extract(ob: &PyAny) -> PyResult<Self> {
        if let Ok(s) = ob.extract::<String>() {
            Ok(Value::String(s))
        } else if let Ok(f) = ob.extract::<f64>() {
            Ok(Value::Number(f))
        } else if let Ok(b) = ob.extract::<bool>() {
            Ok(Value::Boolean(b))
        } else if let Ok(c) = ob.extract::<PyRef<Classifier>>() {
            // Extract the underlying Rust Classifier from the PyRef
            Ok(Value::Classifier(Box::new(c.clone())))
        } else if let Ok(l) = ob.extract::<Py<PyList>>() {
            let mut vec = Vec::new();
            for item in l.as_ref(ob.py()).iter() {
                vec.push(item.extract()?);
            }
            Ok(Value::Array(vec))
        } else {
            Err(PyValueError::new_err("Could not convert Python object to Value"))
        }
    }
}

impl IntoPy<PyObject> for Value {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            Value::String(s) => s.into_py(py),
            Value::Number(f) => f.into_py(py),
            Value::Boolean(b) => b.into_py(py),
            Value::Classifier(c) => c.into_py(py),
            Value::Array(v) => v.into_py(py),
        }
    }
}

// Macro to define the Classifier enum and its methods
macro_rules! define_classifier {
    (
        $enum_name:ident, // The name of the enum (e.g., Classifier)
        $(
            // Each variant definition:
            // $serde_name: The string used for #[serde(rename = "...")]
            // $variant_name: The Rust enum variant name (e.g., REGEXMATCH)
            // $fields: The fields of the variant (e.g., attribute: Attribute, value: Value)
            // $eval_logic: The Rust code for the eval method's match arm
            $serde_name:literal => $variant_name:ident { $($field_name:ident : $field_type:ty),* } => { $($eval_logic:tt)* }
        ),*
    ) => {
        #[pyclass]
        #[derive(Serialize, Deserialize, Debug, Clone)]
        #[serde(tag = "type")]
        pub enum $enum_name {
            $(
                #[serde(rename = $serde_name)]
                $variant_name { $($field_name : $field_type),* },
            )*
        }

        #[pymethods]
        impl $enum_name {
            pub fn eval(&self, py: Python) -> bool { 
                match self {
                    $(
                        $enum_name::$variant_name { $($field_name),* } => {
                            let logic = $($eval_logic)* ;
                            logic(py)
                        }
                    ),*
                }
            }
        }
    };
}

// Define the Classifier enum using the macro
define_classifier!(
    Classifier,
    "re.match" => REGEXMATCH { attribute: Attribute, value: Value } => {
        |py:Python| {
            if let (Value::String(pattern), Ok(attr_py_obj)) = (value, attribute.eval(py)) {
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
    "bool.all" => ALL { attribute: Option<Attribute>, value: Vec<Classifier> } =>  { |py:Python| {  value.iter().all(|c| c.eval(py)) }
    },
    "bool.any" => ANY { attribute: Option<Attribute>, value: Vec<Classifier> } => { |py: Python| { value.iter().any(|c| c.eval(py)) }
    },
    "comparison.lt" => LT { attribute: Attribute, value: Value } => {
        |py:Python| {
        if let (Value::Number(val), Ok(attr_py_obj)) = (value, attribute.eval(py)) {
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
    "comparison.gt" => GT { attribute: Attribute, value: Value } => {
     
        |py:Python| {
        if let (Value::Number(val), Ok(attr_py_obj)) = (value, attribute.eval(py)) {
            if let Ok(attr_val) = attr_py_obj.extract::<f64>(py) {
                attr_val > *val
            } else {
                false
            }
        } else {
            false
        }}
    },
    "comparison.gte" => GTE { attribute: Attribute, value: Value } => {
        |py:Python| {
        if let (Value::Number(val), Ok(attr_py_obj)) = (value, attribute.eval(py)) {
            if let Ok(attr_val) = attr_py_obj.extract::<f64>(py) {
                attr_val >= *val
            } else {
                false
            }
        } else {
            false
        }}
    },
    "comparison.lte" => LTE { attribute: Attribute, value: Value } => {
        |py:Python| {
        if let (Value::Number(val), Ok(attr_py_obj)) = (value, attribute.eval(py)) {
            if let Ok(attr_val) = attr_py_obj.extract::<f64>(py) {
                attr_val <= *val
            } else {
                false
            }
        } else {
            false
        }}
    },
    "comparison.eq" => EQ { attribute: Attribute, value: Value } => {
        |py:Python| {
        let attr_py_obj_res = attribute.eval(py);
        if let Ok(attr_py_obj) = attr_py_obj_res {
            if let Value::Number(val) = value {
                if let Ok(attr_val) = attr_py_obj.extract::<f64>(py) {
                    attr_val == *val
                } else {
                    false
                }
            } else if let Value::String(val) = value {
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
