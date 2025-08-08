use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[macro_export]
macro_rules! PythonEnum {
    (
        $enum_name:ident, // The name of the enum (e.g., Classifier)
        $($serde_name:literal => $variant_name:ident),*
    ) => {
        #[pyclass]
        #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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