use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

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

fn verify_checksum_hexfile<P: AsRef<Path>>(hex_file: P) -> Result<Vec<(usize, String)>> {
    let mut failed_records: Vec<(usize, String)> = Vec::new();

    let lines = BufReader::new(File::open(hex_file)?).lines();

    for (line_no, hex_record) in lines.enumerate() {
        let hex_record = hex_record?;
        let checksum = checksum_record(&hex_record);
        if checksum != 0u8 {
            failed_records.push((line_no + 1, hex_record));
        }
    }

    Ok(failed_records)
}

pub fn run<P: AsRef<Path>>(hex_file: P) -> Result<()> {
    let failed_records = verify_checksum_hexfile(hex_file)?;

    if failed_records.is_empty() {
        eprintln!("CHECKSUM verification Successfull !!");
    } else {
        eprintln!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
        eprintln!("CHECKSUM mismatch in the following hex records:");
        eprintln!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
        eprintln!("{:<8} | {:<}", "Line No", "Hex Record");
        eprintln!("{:<8} | {:<}", "-------", "----------");
        for (line_no, hex_record) in failed_records {
            eprintln!("{:<8} | {:<}", line_no, hex_record);
        }
        eprintln!();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_hexfile() -> Result<()> {
        let hex_file = PathBuf::from(r"./src/test/eof.hex");

        let failed_records = verify_checksum_hexfile(hex_file)?;
        assert_eq!(failed_records.len(), 2);

        Ok(())
    }
}
