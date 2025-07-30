use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use regex::Regex;

use crate::attribute::{Attribute, get_attribute_value};

// Represents the flexible `value` field in a Classifier.
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Classifier(Box<Classifier>),
    Array(Vec<Value>),
}

// Represents the possible types within the `args` array of an Attribute.
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ArgValue {
    String(String),
    Number(f64),
    Boolean(bool),
}

// Represents the main Classifier structure.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Classifier {
    #[serde(rename = "re.match")]
    REGEXMATCH { attribute: Attribute, value: Value },
    #[serde(rename = "bool.all")]
    ALL { attribute: Option<Attribute>, value: Vec<Classifier> },
    #[serde(rename = "bool.any")]
    ANY { attribute: Option<Attribute>, value: Vec<Classifier> },
    #[serde(rename = "comparison.lt")]
    LT { attribute: Attribute, value: Value },
    #[serde(rename = "comparison.gt")]
    GT { attribute: Attribute, value: Value },
    #[serde(rename = "comparison.gte")]
    GTE { attribute: Attribute, value: Value },
    #[serde(rename = "comparison.lte")]
    LTE { attribute: Attribute, value: Value },
    #[serde(rename = "comparison.eq")]
    EQ { attribute: Attribute, value: Value },
}

// Evaluates a classifier and returns true or false.
pub fn eval_classifier(classifier: &Classifier) -> bool {
    match classifier {
        Classifier::REGEXMATCH { attribute, value } => {
            if let (Value::String(pattern), Value::String(hostname)) = (value, &get_attribute_value(attribute)) {
                let re = Regex::new(pattern).unwrap();
                re.is_match(hostname)
            } else {
                false
            }
        }
        Classifier::ALL { value, .. } => {
            value.iter().all(|c| eval_classifier(c))
        }
        Classifier::ANY { value, .. } => {
            value.iter().any(|c| eval_classifier(c))
        }
        Classifier::LT { attribute, value } => {
            if let (Value::Number(attr_val), Value::Number(val)) = (&get_attribute_value(attribute), value) {
                attr_val < val
            } else {
                false
            }
        }
        Classifier::GT { attribute, value } => {
            if let (Value::Number(attr_val), Value::Number(val)) = (&get_attribute_value(attribute), value) {
                attr_val > val
            } else {
                false
            }
        }
        Classifier::GTE { attribute, value } => {
            if let (Value::Number(attr_val), Value::Number(val)) = (&get_attribute_value(attribute), value) {
                attr_val >= val
            } else {
                false
            }
        }
        Classifier::LTE { attribute, value } => {
            if let (Value::Number(attr_val), Value::Number(val)) = (&get_attribute_value(attribute), value) {
                attr_val <= val
            } else {
                false
            }
        }
        Classifier::EQ { attribute, value } => {
            if let (Value::Number(attr_val), Value::Number(val)) = (&get_attribute_value(attribute), value) {
                attr_val == val
            } else if let (Value::String(attr_val), Value::String(val)) = (&get_attribute_value(attribute), value) {
                attr_val == val
            } else {
                false
            }
        }
    }
}
