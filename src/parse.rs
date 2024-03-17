use std::path::PathBuf;

use clap::{command, value_parser, Arg, ArgAction};

pub fn parse_args() -> (Option<PathBuf>, bool) {
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

    let hex_file = matches.get_one::<PathBuf>("hex_file");
    let use_pb = matches.get_flag("progress_bar");

    (hex_file.cloned(), use_pb)
}
