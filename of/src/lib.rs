use pyo3::prelude::*;

#[macro_use]
mod type_enum;
mod attribute;
mod classifier;
mod values;
mod offline_feature;

use attribute::Attribute;
use classifier::Classifier;
use offline_feature::{OfflineFeature, Bucket};
use crate::type_enum::{FeatureType, PythonVersion};

#[pymodule]
fn of(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Bucket>()?;
    m.add_class::<FeatureType>()?;
    m.add_class::<PythonVersion>()?;
    m.add_class::<Attribute>()?;
    m.add_class::<Classifier>()?;
    m.add_class::<OfflineFeature>()?;
    Ok(())
}