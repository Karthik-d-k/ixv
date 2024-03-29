use anyhow::Result;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn count_lines<P: AsRef<Path>>(path: P) -> Result<u64> {
    let mut lines = BufReader::new(File::open(path)?).lines();

    let count = lines.try_fold(0, |acc, line| line.map(|_| acc + 1));

    Ok(count?)
}

fn checksum_record(hex_record: &str) -> u8 {
    let mut sum: u32 = 0;
    let strt_idx = hex_record
        .rfind(':')
        .expect("[ixv error]: Aborting due to invalid hex record")
        + 1;
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

fn verify_checksum_hexfile_without_progress<P: AsRef<Path>>(
    hex_file: P,
) -> Result<Vec<(usize, String)>> {
    let mut failed_records: Vec<(usize, String)> = Vec::new();

    let file = File::open(&hex_file)?;
    let lines = BufReader::new(file).lines();

    for (line_no, hex_record) in lines.enumerate() {
        let hex_record = hex_record?;
        let checksum = checksum_record(&hex_record);
        if checksum != 0u8 {
            failed_records.push((line_no + 1, hex_record));
        }
    }

    Ok(failed_records)
}

fn verify_checksum_hexfile_with_progress<P: AsRef<Path>>(
    hex_file: P,
) -> Result<Vec<(usize, String)>> {
    let num_lines = count_lines(&hex_file)?;
    let mut failed_records: Vec<(usize, String)> = Vec::new();
    let pb = indicatif::ProgressBar::new(num_lines);

    let file = File::open(&hex_file)?;
    let lines = BufReader::new(file).lines();

    for (line_no, hex_record) in lines.enumerate() {
        let hex_record = hex_record?;
        let checksum = checksum_record(&hex_record);
        if checksum != 0u8 {
            failed_records.push((line_no + 1, hex_record));
        }

        pb.inc(1)
    }

    pb.finish();

    Ok(failed_records)
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

pub fn run<P: AsRef<Path>>(hex_file: P, pb: bool) -> Result<()> {
    let failed_records: Vec<(usize, String)> = if pb {
        verify_checksum_hexfile_with_progress(hex_file)?
    } else {
        verify_checksum_hexfile_without_progress(hex_file)?
    };

    if failed_records.is_empty() {
        eprintln!("CHECKSUM verification Successfull !!");
    } else {
        eprintln!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
        eprintln!("CHECKSUM mismatch in the following hex records:");
        eprintln!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
        let table = FailedRecordsTable(failed_records);
        eprintln!("{}", table);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_hexfile_without_progress() -> Result<()> {
        let hex_file = PathBuf::from(r"./src/test/eof.hex");

        let failed_records = verify_checksum_hexfile_without_progress(hex_file)?;
        assert_eq!(failed_records.len(), 2);

        Ok(())
    }

    #[test]
    fn test_hexfile_with_progress() -> Result<()> {
        let hex_file = PathBuf::from(r"./src/test/eof.hex");

        let failed_records = verify_checksum_hexfile_with_progress(hex_file)?;
        assert_eq!(failed_records.len(), 2);

        Ok(())
    }

    #[test]
    fn test_hexfile_withandwithout_progress() -> Result<()> {
        let hex_file = PathBuf::from(r"./src/test/eof.hex");

        let failed_records_with_pb = verify_checksum_hexfile_with_progress(&hex_file)?;
        let failed_records_without_pb = verify_checksum_hexfile_without_progress(&hex_file)?;
        assert_eq!(failed_records_with_pb, failed_records_without_pb);

        Ok(())
    }
}
