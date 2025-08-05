use once_cell::sync::OnceCell;
use pyo3::prelude::*;
use pyo3::types::{PyFloat, PyString};
use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::classifier::{ClassifierValue, Classifier};

use pyo3::class::basic::CompareOp;


macro_rules! AttributeEnum {
    ($enum_name:ident => $result_type:ty,
    $(
        $serde_name:literal => $variant_name:ident { $($field_name:ident : $field_type:ty),* } => { $($eval_logic:tt)* }
    ),*

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
            pub fn __richcmp__(&self, other: ClassifierValue, op: CompareOp) -> Classifier{
                match op {
                    CompareOp::Lt => Classifier::LT {attribute: self.clone(), value: other},
                    CompareOp::Le => Classifier::LTE {attribute: self.clone(), value: other},
                    CompareOp::Eq =>Classifier::EQ {attribute: self.clone(), value: other},
                    CompareOp::Ne =>Classifier::EQ  {attribute: self.clone(), value: other},
                    CompareOp::Gt =>Classifier::GT {attribute: self.clone(), value: other},
                    CompareOp::Ge =>Classifier::GTE {attribute: self.clone(), value: other},
 
                }
            }
        }



    };
}
static SESSION_RANDOM_CACHE: OnceCell<PyObject> = OnceCell::new();

AttributeEnum! {
    // Attribute => PyObject,
    Attribute => PyResult<PyObject>,
    "static.number" => StaticNumber {value: f64} =>  {
        |py:Python| {
            Ok(PyFloat::new(py, *value).into())
        }
    },
    "socket.hostname" => Hostname {} =>  {
        |py:Python | {
            Ok(PyString::new(py, &gethostname::gethostname().into_string().unwrap()).into())

        }
    },
    "random.session" => SessionRandom {} => {
        |py:Python| {
            let random_value = SESSION_RANDOM_CACHE.get_or_init(|| {
                PyFloat::new(py, rand::thread_rng().gen_range(0.0..1.0)).into_py(py)
            });
            Ok(random_value.clone_ref(py))
        }
    }

}

