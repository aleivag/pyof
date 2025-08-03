

macro_rules! TypeEnum {
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


        }

    };
}



// Macro to define the Classifier enum and its methods
macro_rules! define_classifier {
    (
        $enum_name:ident, // The name of the enum (e.g., Classifier)
        $(
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
            pub fn json(&self) -> String {
                serde_json::to_string(&self).unwrap()
            }
        }
    };
}

