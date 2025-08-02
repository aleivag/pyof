use once_cell::sync::OnceCell;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyFloat, PyString};
use rand::Rng;
use serde::{Deserialize, Serialize};

// Macro to define the AttributeType enum and implement its `members` method
macro_rules! define_attribute_type {
    ($enum_name:ident, [
        $($serde_name:literal => $variant:ident),
    *]) => {
        #[pyclass]
        #[derive(Serialize, Deserialize, Debug, Clone)]
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

// Define the AttributeType enum using the macro
define_attribute_type!(
    AttributeType, [
        "socket.hostname" => Hostname,
        "random.session" => SessionRandom
    ]
);

// Global cache for SessionRandom
static SESSION_RANDOM_CACHE: OnceCell<PyObject> = OnceCell::new();

// Represents the `attribute` field in a Classifier.
#[pyclass]
#[derive(Serialize, Deserialize, Debug, Clone)] // Added Clone here
pub struct Attribute {
    pub name: AttributeType,
    #[serde(rename = "type")]
    pub attribute_type: String,
}

#[pymethods]
impl Attribute {
    #[new]
    fn new(name: AttributeType, attribute_type: Option<String>) -> Self {
        Attribute {
            name,
            attribute_type: attribute_type.unwrap_or_else(|| "callable-attribute".to_string()),
        }
    }

    pub fn eval(&self, py: Python) -> PyResult<PyObject> {
        match self.name {
            AttributeType::Hostname => {
                Ok(PyString::new(py, &gethostname::gethostname().into_string().unwrap()).into())
            }
            AttributeType::SessionRandom => {
                let random_value = SESSION_RANDOM_CACHE.get_or_init(|| {
                    PyFloat::new(py, rand::thread_rng().gen_range(0.0..1.0)).into_py(py)
                });
                Ok(random_value.clone_ref(py))
            }
        }
    }
}
