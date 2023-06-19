//! # n64romconvert
//!
//! It's a small tool to help you convert
//! between Nintendo 64 ROM formats,
//! including Byte-Swapped Big Endian
//! (v64), Little endian (n64), and Big
//! Endian (z64), on the CLI.
//!
//! You _can_ use it like a crate, but
//! now the question would be why would you.

use colored::Colorize;
use std::{
    fmt::Display,
    fs,
    io::{BufWriter, Read, Write},
    os::unix::prelude::FileExt,
    path::Path,
    process::exit,
};
use RomType::*;

const ROM_LEN: u32 = 8388608; // 8MiB exactly

#[derive(PartialEq, Eq, Debug)]
/// An enum to represent the different ROM types.
pub enum RomType {
    /// A Byte-Swappled LE ROM (v64)
    ByteSwapped,
    /// A Little-Endian ROM (n64)
    LittleEndian,
    /// A Big-Endian ROM (z64)
    BigEndian,
}

impl RomType {
    fn as_str(&self) -> &str {
        match self {
            ByteSwapped => "v64",
            LittleEndian => "n64",
            BigEndian => "z64",
        }
    }

    /// Create a new RomType from
    /// a string type.
    pub fn from_string<S: AsRef<str>>(s: S) -> Result<RomType, Error> {
        let result = match s.as_ref() {
            "n64" => LittleEndian,
            "z64" => BigEndian,
            "v64" => ByteSwapped,
            _ => return Err(Error("the type you entered was not valid!".into())),
        };

        Ok(result)
    }
}

impl ToString for RomType {
    fn to_string(&self) -> String {
        self.as_str().to_owned()
    }
}

#[derive(Debug)]
/// An error.
pub struct Error(pub String);

impl Error {
    pub fn pretty_panic(&self) {
        println!("{}{}", "error: ".bold().red(), self);
        exit(1)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }

    fn description(&self) -> &str {
        &self.0
    }

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

/// Determine the format of the ROM, given the file path.
/// Returns an error if it did not recognize a supported
/// format.
///
/// Supported formats include:
///  * Big Endian Byteswapped (v64)
///  * Big Endian (z64)
///  * Little Endian (n64)
///
pub fn determine_format<P: AsRef<Path>>(file_path: P) -> Result<RomType, Error> {
    let mut file = fs::File::open(file_path.as_ref())
        .unwrap_or_else(|e| panic!("failed to open the file: {}", e));
    let mut buf: [u8; 4] = [0, 0, 0, 0];

    file.read_exact(&mut buf)
        .unwrap_or_else(|e| panic!("failed to read the first 4 bytes of the ROM file: {}", e));

    use RomType::*;

    let result = match buf {
        [0x37, 0x80, 0x40, 0x12] => ByteSwapped,
        [0x40, 0x12, 0x37, 0x80] => LittleEndian,
        [0x80, 0x37, 0x12, 0x40] => BigEndian,
        _ => return Err(Error("Did not recognize a supported format!".to_owned())),
    };

    Ok(result)
}

/// Swap the endianness of the ROM, taking
/// in 2 paths and a callback for progress.
pub fn endian_swap<P: AsRef<Path>>(input_file: P, output_file: P) {
    let in_file = fs::File::open(input_file)
        .unwrap_or_else(|e| panic!("failed to open the ROM as a file: {}", e));
    let out_file = fs::File::create(output_file)
        .unwrap_or_else(|e| panic!("failed to create the new ROM: {}", e));
    let mut out_file_writer = BufWriter::new(out_file);

    for idx in (0..ROM_LEN).step_by(65536) {
        let mut buf: Vec<u8> = (0..65536).map(|_| 0).collect::<Vec<u8>>();
        in_file.read_at(&mut buf, idx.into()).unwrap();

        let new_bytes = buf
            .chunks(4)
            .flat_map(|chunk| [chunk[3], chunk[2], chunk[1], chunk[0]])
            .collect::<Vec<u8>>();

        out_file_writer.write_all(&new_bytes).unwrap();
    }
}

pub fn byte_swap<P: AsRef<Path>>(input_file: P, output_file: P) {
    let in_file = fs::File::open(input_file).unwrap();

    let out_file = fs::File::create(output_file).unwrap();
    let mut out_file_writer = BufWriter::new(out_file);

    for idx in (0..ROM_LEN).step_by(65536) {
        let mut buf = (0..65536).map(|_| 0).collect::<Vec<u8>>();
        in_file.read_at(&mut buf, idx.into()).unwrap();

        // let swapped_bytes = buf.into_iter().rev().collect::<Vec<u8>>();
        let swapped_bytes = buf
            .chunks(2)
            .flat_map(|chunk| [chunk[1], chunk[0]])
            .collect::<Vec<u8>>();

        out_file_writer.write_all(&swapped_bytes).unwrap();
    }
}

/// Both swap byte pairs and change the
/// endianness of a ROM, taking in 2
/// paths and a callback for progress.
pub fn byte_endian_swap<P: AsRef<Path>>(input_file: P, output_file: P) {
    let in_file = fs::File::open(input_file).unwrap();

    let out_file = fs::File::create(output_file).unwrap();
    let mut out_file_writer = BufWriter::new(out_file);

    for idx in (0..ROM_LEN).step_by(65536) {
        let mut buf: Vec<u8> = (0..65536).map(|_| 0).collect::<Vec<u8>>();
        in_file.read_at(&mut buf, idx.into()).unwrap();

        let new_bytes = buf
            .chunks(4)
            .flat_map(|chunk| [chunk[2], chunk[3], chunk[0], chunk[1]])
            .collect::<Vec<u8>>();

        out_file_writer.write_all(&new_bytes).unwrap();
    }
}
