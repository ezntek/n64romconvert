use clap::Parser;
use colored::Colorize;
use n64romconvert::*;
use std::path::PathBuf;
use RomType::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
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

    let rom_type = determine_format(&args.rom).unwrap_or_else(|err| {
        err.pretty_panic();
        unreachable!()
    });

    let output_type = match &args.output_type {
        Some(s) => RomType::from_string(s),
        None => {
            if let Some(outfile) = &args.outfile {
                let ext = if let Some(ext) = outfile.extension() {
                    ext.to_str().unwrap()
                } else {
                    "z64"
                };

                RomType::from_string(ext)
            } else {
                RomType::from_string("z64")
            }
        }
    }
    .unwrap_or_else(|err| {
        err.pretty_panic();
        unreachable!()
    });

    let target_filename = args
        .outfile
        .unwrap_or(PathBuf::from(format!("rom.{}", output_type.to_string())));

    if output_type == rom_type {
        Error(format!(
            "No need to convert, the ROM type ({}) is already the output type ({})!",
            rom_type.to_string(),
            output_type.to_string(),
        ))
        .pretty_panic();
    };

    // define some callbacks
    let handle_byteswapped = || match rom_type {
        BigEndian => byte_swap(&args.rom, &target_filename),
        LittleEndian => byte_endian_swap(&args.rom, &target_filename),
        ByteSwapped => unreachable!(),
    };
    let handle_le = || match rom_type {
        LittleEndian => unreachable!(),
        BigEndian => endian_swap(&args.rom, &target_filename),
        ByteSwapped => byte_swap(&args.rom, &target_filename),
    };
    let handle_be = || match rom_type {
        LittleEndian => endian_swap(&args.rom, &target_filename),
        BigEndian => unreachable!(),
        ByteSwapped => byte_swap(&args.rom, &target_filename),
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
