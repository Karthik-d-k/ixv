use std::path::PathBuf;

use clap::Parser;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to an hex file
    hex_file: PathBuf,
}

pub fn run() -> Result<()> {
    let args = Args::parse();
    let hex_file = args.hex_file;

    calc_checksum(&hex_file)?;

    Ok(())
}

fn calc_checksum(hex_file: &PathBuf) -> Result<()> {
    let _ = hex_file;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_files() -> Result<()> {
        let hex_file = PathBuf::from(r"./test/eof.hex");

        let _ = calc_checksum(&hex_file);

        Ok(())
    }
}
