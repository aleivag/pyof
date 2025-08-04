

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

