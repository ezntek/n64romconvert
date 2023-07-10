mod bindings;
mod direct_api;

use pyo3::{prelude::*, wrap_pymodule};

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "pyn64romconvert")]
fn pyn64romconvert(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(direct_api::direct_api))?;

    m.add_class::<bindings::Rom>()?;
    m.add_class::<bindings::Converter>()?;

    Ok(())
}
