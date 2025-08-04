use once_cell::sync::OnceCell;
use pyo3::prelude::*;
use pyo3::types::{PyFloat, PyString};
use rand::Rng;
use serde::{Deserialize, Serialize};


static SESSION_RANDOM_CACHE: OnceCell<PyObject> = OnceCell::new();

TypeEnum! {
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

