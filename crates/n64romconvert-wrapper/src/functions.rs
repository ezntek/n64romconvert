use n64romconvert as conv;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyfunction]
pub fn determine_format(path: String) -> PyResult<String> {
    match conv::determine_format(path) {
        Ok(t) => Ok(t.to_string()),
        Err(e) => {
            let err = PyValueError::new_err(e.to_string());
            Err(err)
        }
    }
}

#[pyfunction]
pub fn byte_swap(infile: String, outfile: String) -> PyResult<()> {
    Ok(conv::byte_swap(infile, outfile))
}

#[pyfunction]
pub fn byte_endian_swap(infile: String, outfile: String) -> PyResult<()> {
    Ok(conv::byte_endian_swap(infile, outfile))
}

#[pyfunction]
pub fn endian_swap(infile: String, outfile: String) -> PyResult<()> {
    Ok(conv::endian_swap(infile, outfile))
}
