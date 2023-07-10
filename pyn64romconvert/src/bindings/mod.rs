mod bindings;

pub use bindings::*;

use n64romconvert::RomType;
use pyo3::exceptions::{PyFileNotFoundError, PyValueError};
use pyo3::PyResult;
use std::path::Path;

fn check_if_exists(path: &Path) -> PyResult<()> {
    if !path.exists() {
        let err =
            PyFileNotFoundError::new_err(format!("could not find a ROM at {}", path.display()));
        return Err(err);
    }

    Ok(())
}

fn to_rom_type(s: String) -> PyResult<RomType> {
    match RomType::from_string(s.to_lowercase()) {
        Ok(t) => Ok(t),
        Err(e) => {
            let err = PyValueError::new_err(e.to_string());
            Err(err)
        }
    }
}
