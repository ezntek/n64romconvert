# n64romconvert

It's a tool containing 2 binaries to help you check and convert between Nintendo 64 ROM formats, including Byte-Swapped Big Endian (v64), Little endian (n64), and Big Endian (z64), on the CLI. 

## Installation

You can issue a cargo install like this: `cargo install n64romconvert`, or issue `make`.

## Usage

n64romconvert comes with 2 sub-tools, `n64romconvert` itself, and another binary called `n64romtype`, that uses this crate's `determine_format` function to determine the format of a given ROM.

n64romconvert:

```
Small tool to help you convert between Nintendo 64 ROM formats, on the CLI.

Usage: n64romconvert [OPTIONS] <ROM>

Arguments:
  <ROM>  

Options:
  -T, --output-type <OUTPUT_TYPE>  specify the output ROM type. selects z64 by default.
  -o, --outfile <OUTFILE>          specify the output file.
  -h, --help                       Print help
  -V, --version                    Print version
```

n64romtype:

```
Small tool to help you convert between Nintendo 64 ROM formats, on the CLI.

Usage: n64romtype <FILE_PATH>

Arguments:
  <FILE_PATH>  

Options:
  -h, --help     Print help
  -V, --version  Print version
```