use clap::Parser;
use std::{path::PathBuf, process};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Path to an hex file
    hex_file: PathBuf,
    /// Show progress bar
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pb: bool,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = ixv::run(&args.hex_file, args.pb) {
        eprintln!("[ixv error]: {}", e);

        process::exit(1);
    }
}
