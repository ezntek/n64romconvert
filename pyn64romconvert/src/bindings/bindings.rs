use n64romconvert::*;
use pyo3::prelude::*;
use std::path::PathBuf;

use super::{check_if_exists, to_rom_type};

#[derive(Clone)]
#[pyclass]
pub struct Rom {
    rom_type: RomType,
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
        let rom_type = to_rom_type(rom_type)?;

        let result = Rom {
            rom_type,
            path: path.into(),
        };

        Ok(result)
    }

    pub fn get_rom_type(&self) -> String {
        self.rom_type.to_string()
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

    pub fn to_byteswapped(&self, target_path: String) -> PyResult<()> {
        use RomType::*;
        let target_path: PathBuf = target_path.into();

        check_if_exists(&target_path)?;

        match &self.rom.rom_type {
            BigEndian => byte_swap(&self.rom.path, &target_path),
            LittleEndian => byte_endian_swap(&self.rom.path, &target_path),
            _ => unreachable!(),
        };

        Ok(())
    }

    pub fn to_little_endian(&self, target_path: String) -> PyResult<()> {
        use RomType::*;
        let target_path: PathBuf = target_path.into();

        check_if_exists(&target_path)?;

        match &self.rom.rom_type {
            BigEndian => byte_swap(&self.rom.path, &target_path),
            ByteSwapped => byte_endian_swap(&self.rom.path, &target_path),
            _ => unreachable!(),
        };

        Ok(())
    }

    pub fn to_big_endian(&self, target_path: String) -> PyResult<()> {
        use RomType::*;
        let target_path: PathBuf = target_path.into();

        check_if_exists(&target_path)?;

        match &self.rom.rom_type {
            ByteSwapped => byte_swap(&self.rom.path, &target_path),
            LittleEndian => endian_swap(&self.rom.path, &target_path),
            _ => unreachable!(),
        }

        Ok(())
    }
}
