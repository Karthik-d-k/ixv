use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

const DATA_RECORD: u8 = 0x00;
const EOF_RECORD: u8 = 0x01;

/// line no, reason, record
type HexError = (usize, String, String);

/// Check `:LL AAAA TT DD..DD CC` per Intel HEX Rev A. Returns the record type.
fn check_record(line: &str) -> Result<u8, String> {
    let body = line
        .strip_prefix(':')
        .ok_or("record does not start with ':'")?;
    let body = body.trim_end();
    if body.contains(':') {
        return Err("line holds more than one record".to_string());
    }

    let digits = body.as_bytes();
    if digits.len() < 10 {
        return Err(format!(
            "record has {} hex digits, minimum is 10",
            digits.len()
        ));
    }
    if !digits.len().is_multiple_of(2) {
        return Err(format!("odd number of hex digits ({})", digits.len()));
    }

    let bytes = digits
        .chunks(2)
        .map(|pair| {
            std::str::from_utf8(pair)
                .ok()
                .and_then(|s| u8::from_str_radix(s, 16).ok())
                .ok_or_else(|| format!("invalid hex digits '{}'", String::from_utf8_lossy(pair)))
        })
        .collect::<Result<Vec<u8>, String>>()?;

    let data_len = usize::from(bytes[0]);
    let address = u16::from_be_bytes([bytes[1], bytes[2]]);
    let record_type = bytes[3];
    let checksum = bytes[bytes.len() - 1];

    if bytes.len() != data_len + 5 {
        return Err(format!(
            "length field says {} data byte(s), record carries {}",
            data_len,
            bytes.len() - 5
        ));
    }

    let mandated_len = match record_type {
        DATA_RECORD => None,
        EOF_RECORD => Some(0),
        0x02 | 0x04 => Some(2),
        0x03 | 0x05 => Some(4),
        _ => return Err(format!("undefined record type {:02X}", record_type)),
    };

    // The load offset is only used by data records.
    if record_type != DATA_RECORD && address != 0 {
        return Err(format!(
            "record type {:02X} must have load offset 0000, found {:04X}",
            record_type, address
        ));
    }
    if let Some(mandated_len) = mandated_len {
        if data_len != mandated_len {
            return Err(format!(
                "record type {:02X} must carry {} data byte(s), found {}",
                record_type, mandated_len, data_len
            ));
        }
    }

    // Every byte including CC sums to zero in the low byte.
    let sum = bytes.iter().fold(0u8, |acc, &byte| acc.wrapping_add(byte));
    if sum != 0 {
        return Err(format!(
            "record does not sum to zero; checksum is {:02X}",
            checksum
        ));
    }

    Ok(record_type)
}

fn verify_hexfile<P: AsRef<Path>>(hex_file: P) -> io::Result<Vec<HexError>> {
    let mut failures: Vec<HexError> = Vec::new();
    let mut eof_line: Option<usize> = None;
    let mut last_line = 0;

    for (idx, line) in BufReader::new(File::open(hex_file)?).lines().enumerate() {
        let line = line?;
        let line_no = idx + 1;
        last_line = line_no;

        if line.trim().is_empty() {
            continue;
        }

        match check_record(&line) {
            Err(reason) => failures.push((line_no, reason, line)),
            Ok(record_type) => match eof_line {
                Some(eof) => failures.push((
                    line_no,
                    format!("record follows the EOF record on line {}", eof),
                    line,
                )),
                None if record_type == EOF_RECORD => eof_line = Some(line_no),
                None => {}
            },
        }
    }

    if eof_line.is_none() {
        let reason = "file has no EOF record (:00000001FF)".to_string();
        failures.push((last_line.max(1), reason, String::new()));
    }

    Ok(failures)
}

/// Verify one hex file, reporting to stderr. False if it has failures.
pub fn run<P: AsRef<Path>>(hex_file: P) -> io::Result<bool> {
    let failures = verify_hexfile(hex_file)?;

    if failures.is_empty() {
        eprintln!("Hex file verified !!");
        return Ok(true);
    }

    let width = failures
        .iter()
        .map(|(_, reason, _)| reason.len())
        .max()
        .unwrap_or(6);
    eprintln!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    eprintln!("Verification failed:");
    eprintln!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    eprintln!("{:<8} | {:<width$} | Hex Record", "Line No", "Reason");
    eprintln!("{:<8} | {} | ----------", "-------", "-".repeat(width));
    for (line_no, reason, hex_record) in failures {
        eprintln!("{:<8} | {:<width$} | {}", line_no, reason, hex_record);
    }
    eprintln!();

    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn accepts_valid_records() {
        for record in [
            ":10010000214601360121470136007EFE09D2190140", // data
            ":10010000214601360121470136007efe09d2190140", // lowercase is legal
            ":00000001FF",                                 // EOF
            ":020000021200EA",                             // extended segment address
            ":0400000300003800C1",                         // start segment address
            ":02000004FFFFFC",                             // extended linear address
            ":04000005000000CD2A",                         // start linear address
            ":00000001FF\r",                               // CRLF line ending
        ] {
            assert!(
                check_record(record).is_ok(),
                "rejected valid record: {record}"
            );
        }
    }

    #[test]
    fn rejects_invalid_records() {
        for record in [
            ":10010000214601360121470136007EFE09D2190141", // checksum off by one
            ":00000001FE",                                 // EOF with wrong checksum
            ":ZZZZZZZZZZZZ",                               // not hex at all
            ":",                                           // no fields
            "no record mark here",                         // no ':'
            "comment:00000001FF",                          // text ahead of the record mark
            ":1001000021460136012147013600D219014",        // odd digit count
            ":10010000AABB8A",                             // length field lies
            ":01010000AABBCCDDF0",                         // length field lies, other way
            ":00000009F7",                                 // undefined record type
            ":00FFFF09F9",                                 // undefined type, reported as such
            ":04000001DEADBEEFC3",                         // EOF must carry no data
            ":00FFFF0101",                                 // EOF must have offset 0000
            ":02FFFF0412000A",                             // type 04 must have offset 0000
            ":030000041200EA",                             // type 04 must carry 2 bytes
            ":10010000214601360121470136007EFE09D2190141:00000001FF", // lost newline
        ] {
            assert!(
                check_record(record).is_err(),
                "accepted invalid record: {record}"
            );
        }
    }

    #[test]
    fn reports_checksum_failures_and_eof_ordering() -> io::Result<()> {
        let failures = verify_hexfile(PathBuf::from(r"./tests/data/eof.hex"))?;

        assert_eq!(failures.len(), 3);
        assert!(failures[0].1.contains("follows the EOF record"));
        assert!(failures[1].1.contains("checksum"));
        assert!(failures[2].1.contains("checksum"));

        Ok(())
    }

    #[test]
    fn accepts_a_well_formed_file() -> io::Result<()> {
        let failures = verify_hexfile(PathBuf::from(r"./tests/data/valid.hex"))?;
        assert!(failures.is_empty(), "{failures:?}");

        Ok(())
    }

    #[test]
    fn requires_an_eof_record() -> io::Result<()> {
        let failures = verify_hexfile(PathBuf::from(r"./tests/data/no_eof.hex"))?;

        assert_eq!(failures.len(), 1);
        assert!(failures[0].1.contains("no EOF record"));

        Ok(())
    }
}
