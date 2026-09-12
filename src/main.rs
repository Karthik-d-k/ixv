use std::{env, process};

const USAGE: &str = "\
A CLI application for verifying intel hex file

Usage: ixv <HEX_FILE>

Arguments:
  <HEX_FILE>  Path to the hex file

Options:
  -h, --help     Print help
  -V, --version  Print version";

fn main() {
    let mut args = env::args_os().skip(1);
    let hex_file = match args.next() {
        Some(arg) => arg,
        None => usage_error("no hex file provided"),
    };

    if hex_file == "-h" || hex_file == "--help" {
        println!("{USAGE}");
        return;
    }
    if hex_file == "-V" || hex_file == "--version" {
        println!("ixv {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    let shown = hex_file.to_string_lossy();
    if shown.starts_with('-') {
        usage_error(&format!("unknown option '{}'", shown));
    }
    if args.next().is_some() {
        usage_error("expected exactly one hex file");
    }

    match ixv::run(&hex_file) {
        Ok(true) => {}
        Ok(false) => process::exit(1),
        Err(e) => {
            eprintln!("[ixv error]: {}", e);

            process::exit(1);
        }
    }
}

fn usage_error(reason: &str) -> ! {
    eprintln!("[ixv error]: {}\n\n{}", reason, USAGE);

    process::exit(2);
}
