use pyo3::prelude::*;
use crate::values::FeatureValue;

#[pyclass]
#[derive(Debug, Clone, PartialEq)]
pub enum EvalResultType {
    Ok,
    Default,
    NotExist,
    Error,
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct EvalResult {
    #[pyo3(get)]
    pub result_type: EvalResultType,
    #[pyo3(get)]
    pub bucket_name: Option<String>,
    #[pyo3(get)]
    pub value: FeatureValue,
}

#[pymethods]
impl EvalResult {
    fn __repr__(&self) -> String {
        format!(
            "EvalResult(result_type={:?}, bucket_name={:?}, value={:?})",
            self.result_type,
            self.bucket_name,
            self.value
        )
    }
}

impl EvalResult {
    pub fn new(result_type: EvalResultType, bucket_name: Option<String>, value: FeatureValue) -> Self {
        EvalResult {
            result_type,
            bucket_name,
            value,
        }
    }
}