use std::{path::PathBuf, process};

use clap::{command, value_parser, Arg, ArgAction};

fn main() {
    let matches = command!()
        .arg(
            Arg::new("hex_file")
                .help("Path to an hex file")
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("progress_bar")
                .short('p')
                .long("progress")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let hex_file = matches
        .get_one::<PathBuf>("hex_file")
        .expect("Path to an hex file is missing");
    let use_pb = matches.get_flag("progress_bar");

    if let Err(e) = ixv::run(hex_file, use_pb) {
        eprintln!("[ixv error]: {}", e);

        process::exit(1);
    }
}
