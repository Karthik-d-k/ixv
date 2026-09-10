use clap::Parser;
use std::{path::PathBuf, process};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Path to one or more hex file(s)
    #[arg(required = true)]
    hex_file: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();

    for hex_file in args.hex_file {
        println!("Hex File: {}", hex_file.display());
        if let Err(e) = ixv::run(hex_file) {
            eprintln!("[ixv error]: {}", e);

            process::exit(1);
        }
    }
}
