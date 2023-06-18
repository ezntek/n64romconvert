use clap::Parser;
use colored::Colorize;
use n64romconvert::*;
use std::{io::Write, path::PathBuf};
use RomType::*;

#[derive(Parser)]
struct Args {
    rom: PathBuf,

    #[arg(
        short = 'T',
        long,
        help = "specify the output ROM type. selects z64 by default."
    )]
    output_type: Option<String>,

    #[arg(short, long, help = "specify the output file.")]
    outfile: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let rom_type = determine_format(&args.rom).unwrap();
    let output_type =
        RomType::from_str(args.output_type.unwrap_or("z64".to_owned()).to_lowercase()).unwrap();

    let target_filename = args
        .outfile
        .unwrap_or(PathBuf::from(format!("rom.{}", output_type.to_string())));

    if output_type == rom_type {
        panic!(
            "No need to convert, the ROM type ({}) is already the output type ({})!",
            rom_type.to_string(),
            output_type.to_string()
        );
    };
    let progress_cb = |progress: f64| {
        let mut stdout = std::io::stdout();

        stdout
            .write(format!("{}% done\r", (progress * (100 as f64)) as u16).as_bytes())
            .expect("failed to draw the progress bar..?");
    };

    // define some callbacks
    let handle_byteswapped = || match rom_type {
        LittleEndian => byte_swap(&args.rom, &target_filename, progress_cb),
        BigEndian => byte_endian_swap(&args.rom, &target_filename, progress_cb),
        ByteSwapped => unreachable!(),
    };
    let handle_le = || match rom_type {
        LittleEndian => unreachable!(),
        BigEndian => endian_swap(&args.rom, &target_filename, progress_cb),
        ByteSwapped => byte_swap(&args.rom, &target_filename, progress_cb),
    };
    let handle_be = || match rom_type {
        LittleEndian => endian_swap(&args.rom, &target_filename, progress_cb),
        BigEndian => unreachable!(),
        ByteSwapped => byte_endian_swap(&args.rom, &target_filename, progress_cb),
    };

    println!("{}        {:?}", "rom type:".bold().cyan(), rom_type);
    println!("{}        {:?}", "out type:".bold().cyan(), output_type);
    println!(
        "{}   {}",
        "new file name:".bold().green(),
        target_filename.display()
    );

    match output_type {
        ByteSwapped => handle_byteswapped(),
        LittleEndian => handle_le(),
        BigEndian => handle_be(),
    }
}
