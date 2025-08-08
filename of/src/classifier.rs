use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyList;
use serde::{Deserialize, Serialize};

use crate::attribute::Attribute;

use regex::Regex;

macro_rules! PyTagEnum{
    (
        $enum_name:ident,
        $($serde_name:literal => $variant_name:ident { $($field_name:ident : $field_type:ty),* }),*
    ) => {

        #[pyclass]
        #[derive(Serialize, Deserialize, Debug, Clone)]
        #[serde(tag = "__type")]
        pub enum $enum_name {
            $(
                #[serde(rename = $serde_name)]
                $variant_name { $($field_name : $field_type),* },
            )*
        }


    }
}


macro_rules! ClassifierEnum {
    ($enum_name:ident => $result_type:ty,
    $(
        $serde_name:literal => $variant_name:ident { $($field_name:ident : $field_type:ty),* } => { $($eval_logic:tt)* }
    ),*

    ) => {

        PyTagEnum! {
            $enum_name,
            $(
                $serde_name => $variant_name { $($field_name : $field_type),* }
            ),*

        }

        #[pymethods]
        impl $enum_name { 

            pub fn eval(&self, py: Python) -> $result_type  {
                match self {
                    $(
                        $enum_name::$variant_name { $($field_name),* } => {
                            let logic = $($eval_logic)* ;
                            logic(py)
                        }
                    ),*
                }
            }
            pub fn json(&self) -> String {
                serde_json::to_string(&self).unwrap()
            }

        }

    };
}


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

ClassifierEnum!(
    Classifier=>PyResult<bool>,
    "re.match" => REGEXMATCH { attribute: Attribute, value: ClassifierValue } => {
        |py:Python| {
            let attr_py_obj = attribute.eval(py)?;
            if let ClassifierValue::String(pattern) = value {
                if let Ok(hostname) = attr_py_obj.extract::<String>(py) {
                    let re = Regex::new(pattern).map_err(|e| PyValueError::new_err(e.to_string()))?;
                    Ok(re.is_match(&hostname))
                } else {
                    Ok(false)
                }
            } else {
                Ok(false)
            }
        }
    },
    "bool.all" => ALL { value: Vec<Classifier> } =>  {
        |py:Python| {
            for c in value {
                if !c.eval(py)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }
    },
    "bool.any" => ANY { value: Vec<Classifier> } => {
        |py: Python| {
            for c in value {
                if c.eval(py)? {
                    return Ok(true);
                }
            }
            Ok(false)
        }
    },
    "bool.none" => NOT {value: ClassifierValue} => {
        |py:Python| {
            match value {
                ClassifierValue::Classifier(c) => Ok(!c.eval(py)?),
                ClassifierValue::String(s) => Ok(s.is_empty()),
                ClassifierValue::Number(f) => Ok(*f == 0.0),
                ClassifierValue::Boolean(b) => Ok(!*b),
                ClassifierValue::Array(v) => Ok(v.is_empty()),
            }
        }
    },
    "comparison.lt" => LT { attribute: Attribute, value: ClassifierValue } => {
        |py:Python| {
            let attr_py_obj = attribute.eval(py)?;
            if let ClassifierValue::Number(val) = value {
                if let Ok(attr_val) = attr_py_obj.extract::<f64>(py) {
                    Ok(attr_val < *val)
                } else {
                    Ok(false)
                }
            } else {
                Ok(false)
            }
        }
    },
    "comparison.gt" => GT { attribute: Attribute, value: ClassifierValue } => {
        |py:Python| {
            let attr_py_obj = attribute.eval(py)?;
            if let ClassifierValue::Number(val) = value {
                if let Ok(attr_val) = attr_py_obj.extract::<f64>(py) {
                    Ok(attr_val > *val)
                } else {
                    Ok(false)
                }
            } else {
                Ok(false)
            }
        }
    },
    "comparison.gte" => GTE { attribute: Attribute, value: ClassifierValue } => {
        |py:Python| {
            let attr_py_obj = attribute.eval(py)?;
            if let ClassifierValue::Number(val) = value {
                if let Ok(attr_val) = attr_py_obj.extract::<f64>(py) {
                    Ok(attr_val >= *val)
                } else {
                    Ok(false)
                }
            } else {
                Ok(false)
            }
        }
    },
    "comparison.lte" => LTE { attribute: Attribute, value: ClassifierValue } => {
        |py:Python| {
            let attr_py_obj = attribute.eval(py)?;
            if let ClassifierValue::Number(val) = value {
                if let Ok(attr_val) = attr_py_obj.extract::<f64>(py) {
                    Ok(attr_val <= *val)
                } else {
                    Ok(false)
                }
            } else {
                Ok(false)
            }
        }
    },
    "comparison.eq" => EQ { attribute: Attribute, value: ClassifierValue } => {
        |py:Python| {
            let attr_py_obj = attribute.eval(py)?;
            if let ClassifierValue::Number(val) = value {
                if let Ok(attr_val) = attr_py_obj.extract::<f64>(py) {
                    Ok(attr_val == *val)
                } else {
                    Ok(false)
                }
            } else if let ClassifierValue::String(val) = value {
                if let Ok(attr_val) = attr_py_obj.extract::<String>(py) {
                    Ok(attr_val == *val)
                } else {
                    Ok(false)
                }
            } else {
                Ok(false)
            }
        }
    }

);
