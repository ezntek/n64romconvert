use n64romconvert::*;
use pyo3::exceptions::{PyFileNotFoundError, PyValueError};
use pyo3::prelude::*;
use std::path::{Path, PathBuf};

#[pyclass]
#[derive(Clone)]
enum PyRomType {
    BigEndian,
    ByteSwapped,
    LittleEndian,
}

fn check_if_exists(path: &Path) -> Option<PyErr> {
    if !path.exists() {
        let err =
            PyFileNotFoundError::new_err(format!("could not find a ROM at {}", path.display()));
        return Some(err);
    }

    None
}

#[derive(Clone)]
#[pyclass(name = "n64romconvert")]
pub struct Rom {
    #[pyo3(get)]
    rom_type: PyRomType,
    #[pyo3(get)]
    path: PathBuf,
}

#[pymethods]
impl Rom {
    /// Creates a new `[Rom]`.
    ///
    /// It takes in a region as a
    /// string, either `"us"`, `"eu"`,
    /// or "jp".
    #[new]
    pub fn new(rom_type: String, path: String) -> PyResult<Rom> {
        use PyRomType::*;

        let rom_type = match rom_type.as_str() {
            "z64" => BigEndian,
            "v64" => ByteSwapped,
            "n64" => LittleEndian,
            _ => {
                return Err(PyValueError::new_err(
                    "The ROM type must be 'z64', 'v64' or 'n64'!",
                ))
            }
        };

        let result = Rom {
            rom_type,
            path: path.into(),
        };

        Ok(result)
    }
}

#[pyclass]
pub struct Converter {
    rom: Rom,
}

#[pymethods]
impl Converter {
    #[new]
    pub fn new(rom: Rom) -> PyResult<Converter> {
        let result = Converter { rom };

        Ok(result)
    }

    pub fn endian_swap(&self, target_path: String) -> PyResult<()> {
        let path = PathBuf::from(target_path);

        if let Some(e) = check_if_exists(&path) {
            return Err(e);
        }

        endian_swap(&self.rom.path, &path);

        Ok(())
    }

    pub fn byte_swap(&self, target_path: String) -> PyResult<()> {
        let path = PathBuf::from(target_path);

        if let Some(e) = check_if_exists(&path) {
            return Err(e);
        }

        byte_swap(&self.rom.path, &path);

        Ok(())
    }

    pub fn byte_endian_swap(&self, target_path: String) -> PyResult<()> {
        let path = PathBuf::from(target_path);

        if let Some(e) = check_if_exists(&path) {
            return Err(e);
        }

        byte_endian_swap(&self.rom.path, &path);

        Ok(())
    }
}
