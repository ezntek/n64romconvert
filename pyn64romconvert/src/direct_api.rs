use n64romconvert as conv;
use pyo3::{exceptions::PyFileNotFoundError, prelude::*};
use std::path::PathBuf;

fn exists(path: PathBuf) -> PyResult<PathBuf> {
    if !path.exists() {
        let err =
            PyFileNotFoundError::new_err(format!("could not find a file at {}!", path.display()));
        Err(err)
    } else {
        Ok(path)
    }
}

#[pyfunction]
pub fn byte_swap(from: PathBuf, to: PathBuf) -> PyResult<()> {
    conv::byte_swap(exists(from)?, exists(to)?);

    Ok(())
}

#[pyfunction]
pub fn byte_endian_swap(from: PathBuf, to: PathBuf) -> PyResult<()> {
    conv::byte_endian_swap(exists(from)?, exists(to)?);

    Ok(())
}

#[pyfunction]
pub fn endian_swap(from: PathBuf, to: PathBuf) -> PyResult<()> {
    conv::endian_swap(exists(from)?, exists(to)?);

    Ok(())
}

#[pymodule]
pub fn direct_api(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(byte_swap, m)?)?;
    m.add_function(wrap_pyfunction!(byte_endian_swap, m)?)?;
    m.add_function(wrap_pyfunction!(endian_swap, m)?)?;

    Ok(())
}
