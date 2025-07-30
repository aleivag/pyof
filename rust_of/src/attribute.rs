use serde::{Deserialize, Serialize};
use rand::Rng;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::classifier::Value;

// Macro to define the AttributeType enum and implement its `members` method
macro_rules! define_attribute_type {
    ($enum_name:ident, [
        $($serde_name:literal => $variant:ident),
    *]) => {
        #[pyclass]
        #[derive(Serialize, Deserialize, Debug)]
        pub enum $enum_name {
            $(#[serde(rename = $serde_name)]
            $variant,)*
        }

        #[pymethods]
        impl $enum_name {
            #[staticmethod]
            fn members(py: Python) -> PyResult<Py<PyDict>> {
                let dict = PyDict::new(py);
                $(dict.set_item($serde_name, $enum_name::$variant.into_py(py))?;
                )*
                Ok(dict.into())
            }
        }
    };
}

// Represents the possible types within the `args` array of an Attribute.
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ArgValue {
    String(String),
    Number(f64),
    Boolean(bool),
}

// Define the AttributeType enum using the macro
define_attribute_type!(
    AttributeType, [
        "socket.hostname" => Hostname,
        "random.session" => SessionRandom
    ]
);

// Represents the `attribute` field in a Classifier.
#[pyclass]
#[derive(Serialize, Deserialize, Debug)]
pub struct Attribute {
    pub name: AttributeType, 
    #[serde(rename = "type")]
    pub attribute_type: String,
    #[serde(default)]
    pub args: Option<Vec<ArgValue>>,
}

// Gets the value of an attribute.
pub fn get_attribute_value(attribute: &Attribute) -> Value {
    match attribute.name {
        AttributeType::Hostname => Value::String(gethostname::gethostname().into_string().unwrap()),
        AttributeType::SessionRandom => Value::Number(rand::thread_rng().gen_range(0.0..1.0)),
    }
}