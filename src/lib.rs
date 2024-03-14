use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to an hex file
    hex_file: PathBuf,
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).context("Error opening file")?;
    Ok(BufReader::new(file).lines())
}

fn checksum_record(hex_record: &str) -> u8 {
    let mut sum: u32 = 0;
    let strt_idx = hex_record.rfind(':').unwrap() + 1;
    let (_comment, hex_record) = hex_record.split_at(strt_idx);

    // Convert the string into a vector of u8 bytes (in chucks of 2)
    let bytes = hex_record
        .as_bytes()
        .chunks(2)
        .filter_map(|chunk| u8::from_str_radix(std::str::from_utf8(chunk).unwrap_or(""), 16).ok())
        .collect::<Vec<_>>();

    // Calculate the sum of the hexadecimal values
    for byte in bytes {
        sum += u32::from(byte);
    }

    // Get the least significant byte (LSB) of the sum
    (sum & 0xFF) as u8
}

fn verify_checksum_hexfile(hex_file: &PathBuf) -> Result<()> {
    if let Ok(lines) = read_lines(hex_file) {
        for (line_no, hex_record) in lines.map_while(Result::ok).enumerate() {
            let checksum = checksum_record(&hex_record);
            if checksum != 0u8 {
                eprintln!("CHECKSUM ERROR :(");
                eprintln!("[line]: hex_record");
                eprintln!("[{:^4}]: {}", line_no + 1, hex_record);
            }
        }
    }

    Ok(())
}

pub fn run() -> Result<()> {
    let args = Args::parse();
    let hex_file = args.hex_file;

    verify_checksum_hexfile(&hex_file)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eof_hex() -> Result<()> {
        let hex_file = PathBuf::from(r"./test/eof.hex");

        let _ = verify_checksum_hexfile(&hex_file);

        Ok(())
    }
}
