use pyo3::prelude::*;
use std::fs;

#[macro_use]
mod type_enum;
mod attribute;
mod classifier;
mod values;
mod offline_feature;
mod result;

use attribute::Attribute;
use classifier::Classifier;
use offline_feature::{OfflineFeature, Bucket};
use crate::type_enum::{FeatureType, PythonVersion};
use values::FeatureValue;
use result::{EvalResult, EvalResultType};

const MATERIALIZED_FEATURES_PATH: &str = "features/materialized";

#[pyfunction]
fn eval(py: Python, feature_name: &str, default_value: FeatureValue) -> PyResult<EvalResult> {
    let path = format!("{}/{}.json", MATERIALIZED_FEATURES_PATH, feature_name);
    match fs::read_to_string(path) {
        Ok(content) => {
            match OfflineFeature::loads(py, &content) {
                Ok(of) => of.get_bucket_name_and_value(py),
                Err(_) => Ok(EvalResult::new(EvalResultType::Error, None, default_value)),
            }
        }
        Err(_) => Ok(EvalResult::new(EvalResultType::NotExist, None, default_value)),
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
    m.add_class::<EvalResult>()?;
    m.add_class::<EvalResultType>()?;
    m.add_function(wrap_pyfunction!(eval, m)?)?;
    Ok(())
}