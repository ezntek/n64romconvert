mod functions;

use functions as wrap;
use pyo3::prelude::*;

#[pymodule]
#[pyo3(name = "_n64romconvert_wrapper")]
fn n64romconvert_wrapper(py: Python, module: &PyModule) -> PyResult<()> {
    // the panic exception
    module.add(
        "RustPanicException",
        <pyo3::panic::PanicException as pyo3::PyTypeInfo>::type_object(py),
    )?;

    module.add_function(wrap_pyfunction!(wrap::determine_format, module)?)?;
    module.add_function(wrap_pyfunction!(wrap::byte_swap, module)?)?;
    module.add_function(wrap_pyfunction!(wrap::byte_endian_swap, module)?)?;
    module.add_function(wrap_pyfunction!(wrap::endian_swap, module)?)?;

    Ok(())
}
