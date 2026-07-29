//! # n64romconvert
//!
//! It's a small tool to help you convert
//! between Nintendo 64 ROM formats,
//! including Byte-Swapped Big Endian
//! (v64), Little endian (n64), and Big
//! Endian (z64), on the CLI.
//!

use colored::Colorize;
use std::{
    fmt::Display,
    fs,
    io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write},
    path::Path,
    process::exit,
};

use serde::{Deserialize, Serialize};

use RomType::*;

#[derive(PartialEq, Eq, Debug, Deserialize, Serialize, Clone, Copy)]
pub enum RomType {
    /// A Byte-Swappled LE ROM (v64)
    #[serde(rename = "rom_ByteSwapped")]
    ByteSwapped,
    /// A Little-Endian ROM (n64)
    #[serde(rename = "rom_LittleEndian")]
    LittleEndian,
    /// A Big-Endian ROM (z64)
    #[serde(rename = "rom_BigEndian")]
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

/// Swap the endianness of the ROM.
/// Z64-N64
pub fn endian_swap<P: AsRef<Path>>(input_file: P, output_file: P) {
    let in_file = fs::File::open(input_file)
        .unwrap_or_else(|e| panic!("failed to open the ROM as a file: {}", e));
    let out_file = fs::File::create(output_file)
        .unwrap_or_else(|e| panic!("failed to create the new ROM: {}", e));

    let mut in_reader = BufReader::new(in_file);
    let mut out_writer = BufWriter::new(out_file);

    const CHUNK_COUNT: usize = 256;
    const BUF_SIZE: usize = CHUNK_COUNT * 4;

    let size = in_reader.seek(SeekFrom::End(0)).unwrap();
    if !size.is_multiple_of(CHUNK_COUNT as u64) {
        panic!("ROM size is not a multiple of {BUF_SIZE}!");
    }
    in_reader.seek(SeekFrom::Start(0)).unwrap();

    let mut old_chunk: [u8; BUF_SIZE] = [0; BUF_SIZE];
    let mut new_chunk: [u8; BUF_SIZE] = [0; BUF_SIZE];

    loop {
        let bytes_read = in_reader.read(&mut old_chunk).unwrap();
        if bytes_read == 0 {
            break;
        }
        for chunk in 0..CHUNK_COUNT {
            for i in 0..4 {
                new_chunk[4 * chunk + i] = old_chunk[4 * chunk + (3 - i)];
            }
        }
        out_writer.write_all(&new_chunk).unwrap();
    }
}

/// Byteswap a rom, where pairs of bytes
/// are swapped. Z64-V64
pub fn byte_swap<P: AsRef<Path>>(input_file: P, output_file: P) {
    let in_file = fs::File::open(input_file)
        .unwrap_or_else(|e| panic!("failed to open the ROM as a file: {}", e));
    let out_file = fs::File::create(output_file)
        .unwrap_or_else(|e| panic!("failed to create the new ROM: {}", e));

    let mut in_reader = BufReader::new(in_file);
    let mut out_writer = BufWriter::new(out_file);

    const CHUNK_COUNT: usize = 256;
    const BUF_SIZE: usize = CHUNK_COUNT * 2;

    let size = in_reader.seek(SeekFrom::End(0)).unwrap();
    if !size.is_multiple_of(CHUNK_COUNT as u64) {
        panic!("ROM size is not a multiple of {BUF_SIZE}!");
    }
    in_reader.seek(SeekFrom::Start(0)).unwrap();

    let mut old_chunk: [u8; BUF_SIZE] = [0; BUF_SIZE];
    let mut new_chunk: [u8; BUF_SIZE] = [0; BUF_SIZE];

    loop {
        let bytes_read = in_reader.read(&mut old_chunk).unwrap();
        if bytes_read == 0 {
            break;
        }
        for chunk in 0..CHUNK_COUNT {
            new_chunk[2 * chunk] = old_chunk[2 * chunk + 1];
            new_chunk[2 * chunk + 1] = old_chunk[2 * chunk];
        }
        out_writer.write_all(&new_chunk).unwrap();
    }
}

/// Both swap byte pairs and change the
/// endianness of a ROM.
pub fn byte_endian_swap<P: AsRef<Path>>(input_file: P, output_file: P) {
    let in_file = fs::File::open(input_file)
        .unwrap_or_else(|e| panic!("failed to open the ROM as a file: {}", e));
    let out_file = fs::File::create(output_file)
        .unwrap_or_else(|e| panic!("failed to create the new ROM: {}", e));

    let mut in_reader = BufReader::new(in_file);
    let mut out_writer = BufWriter::new(out_file);

    const CHUNK_COUNT: usize = 256;
    const BUF_SIZE: usize = CHUNK_COUNT * 4;

    let size = in_reader.seek(SeekFrom::End(0)).unwrap();
    if !size.is_multiple_of(CHUNK_COUNT as u64) {
        panic!("ROM size is not a multiple of {BUF_SIZE}!");
    }
    in_reader.seek(SeekFrom::Start(0)).unwrap();

    let mut old_chunk: [u8; BUF_SIZE] = [0; BUF_SIZE];
    let mut new_chunk: [u8; BUF_SIZE] = [0; BUF_SIZE];

    loop {
        let bytes_read = in_reader.read(&mut old_chunk).unwrap();
        if bytes_read == 0 {
            break;
        }
        for chunk in 0..CHUNK_COUNT {
            new_chunk[4 * chunk] = old_chunk[4 * chunk + 2];
            new_chunk[4 * chunk + 1] = old_chunk[4 * chunk + 3];
            new_chunk[4 * chunk + 2] = old_chunk[4 * chunk];
            new_chunk[4 * chunk + 3] = old_chunk[4 * chunk + 1];
        }
        out_writer.write_all(&new_chunk).unwrap();
    }
}
