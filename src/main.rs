use clap::Parser;
use std::{path::PathBuf, process};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Path to one or more hex file(s)
    #[arg(action = clap::ArgAction::Append)]
    hex_file: Vec<PathBuf>,
    /// Show progress bar
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pb: bool,
}

fn main() {
    let args = Args::parse();
    let hex_files = args.hex_file;

    if hex_files.is_empty() {
        eprintln!("[ixv error]: Hex file(s) are not not provided !!\n");
        eprintln!("Usage: ixv [OPTIONS] [HEX_FILE]...\n");
        eprintln!("For more information, try '--help'.");

        process::exit(1);
    }

    for hex_file in hex_files {
        println!("Hex File: {}", hex_file.display());
        if let Err(e) = ixv::run(hex_file, args.pb) {
            eprintln!("[ixv error]: {}", e);

            process::exit(1);
        }
    }
}
