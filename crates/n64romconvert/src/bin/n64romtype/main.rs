use clap::Parser;
use colored::Colorize;
use n64romconvert::{determine_format, Error};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    file_path: PathBuf,
}

fn main() {
    let args = Args::parse();
    let rom_t = determine_format(args.file_path).unwrap_or_else(|e| {
        Error(format!(
            "could not determine the file type of the ROM: {}",
            e
        ))
        .pretty_panic();
        unreachable!();
    });

    println!(
        "{}{:?} ({})",
        "type: ".cyan().bold(),
        rom_t,
        rom_t.to_string().bold()
    )
}
