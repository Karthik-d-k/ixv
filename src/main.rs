mod parse;

use std::process;

use parse::parse_args;

fn main() {
    if let (Some(hex_file), use_pb) = parse_args() {
        if let Err(e) = ixv::run(&hex_file, use_pb) {
            eprintln!("[ixv error]: {}", e);

            process::exit(1);
        }
    } else {
        eprintln!("Usage: ixv [OPTIONS] [hex_file]");
        eprintln!("For more information, try '--help'");

        process::exit(1);
    }
}
