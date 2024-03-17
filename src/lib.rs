use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

use anyhow::{Context, Result};

fn count_lines(path: &Path) -> Result<u64> {
    let mut lines = BufReader::new(File::open(path)?).lines();

    let count = lines.try_fold(0, |acc, line| line.map(|_| acc + 1));

    Ok(count?)
}

fn read_lines(filename: &Path) -> Result<Lines<BufReader<File>>> {
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

fn verify_checksum_hexfile(hex_file: &Path, use_pb: bool) -> Result<()> {
    let num_lines = count_lines(hex_file)?;
    let mut failed_records: Vec<(usize, String)> = Vec::new();
    let pb = indicatif::ProgressBar::new(num_lines);

    if let Ok(lines) = read_lines(hex_file) {
        for (line_no, hex_record) in lines.map_while(Result::ok).enumerate() {
            let checksum = checksum_record(&hex_record);
            if checksum != 0u8 {
                failed_records.push((line_no + 1, hex_record));
            }

            if use_pb {
                pb.inc(1)
            };
        }
    }

    if use_pb {
        pb.finish()
    };

    if !failed_records.is_empty() {
        eprintln!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
        eprintln!("CHECKSUM mismatch in the following hex records:");
        eprintln!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
        let table = FailedRecordsTable(failed_records);
        eprintln!("{}", table);
    }

    Ok(())
}

struct FailedRecordsTable(Vec<(usize, String)>);

impl fmt::Display for FailedRecordsTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut table = String::new();
        table.push_str(&format!("{:<8} | {:<}\n", "Line No", "Hex Record"));
        table.push_str(&format!("{:<8} | {:<}\n", "-------", "----------"));

        for (line_no, hex_record) in &self.0 {
            table.push_str(&format!("{:<8} | {:<}\n", line_no, hex_record));
        }

        write!(f, "{}", table)
    }
}

pub fn run(hex_file: &Path, use_pb: bool) -> Result<()> {
    verify_checksum_hexfile(hex_file, use_pb)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_eof_hex() -> Result<()> {
        let hex_file = PathBuf::from(r"./test/eof.hex");

        let _ = verify_checksum_hexfile(&hex_file);

        Ok(())
    }
}
