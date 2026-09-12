[![Crates.io Version](https://img.shields.io/crates/v/ixv)](https://crates.io/crates/ixv) ![Crates.io Total Downloads](https://img.shields.io/crates/d/ixv) ![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)
-------------

# ixv

:crab: A CLI application for verifying intel hex file.

Checks every record against the [spec](#references): structure, record type, load offset, checksum and the EOF record.

## Install

#### Binaries
1. Binaries are placed in the [releases](https://github.com/Karthik-d-k/ixv/releases), you can choose depending on your OS.
2. Download and extract the binary and you are all set to use the tool.

#### Cargo
```bash
$ cargo install ixv
```


## Usage
```bash
$ ixv <HEX_FILE>
```

**Arguments:**
```
<HEX_FILE>  Path to the hex file
```

**Options:**
```
-h, --help     Print help
-V, --version  Print version
```

**Exit codes:**
```
0: every record verified
1: the file failed verification, or could not be read
2: bad command line
```

## The format

An ASCII encoding of a binary memory image, one record per line:

```
:  10   0100   00   214601360121470136007EFE09D21901   40
   LL   AAAA   TT   DD..DD                             CC
```

| field | bytes | meaning |
|-------|-------|---------|
| `:`    | 1 | record mark; starts the line |
| `LL`   | 1 | number of info/data bytes |
| `AAAA` | 2 | load offset; `0000` for every type other than `00` |
| `TT`   | 1 | record type |
| `DD`   | n | the data |
| `CC`   | 1 | checksum |

| type | record | data |
|------|--------|------|
| `00` | Data | image bytes |
| `01` | End of File | none, always `:00000001FF` |
| `02` | Extended Segment Address | 2 bytes |
| `03` | Start Segment Address | 4 bytes |
| `04` | Extended Linear Address | 2 bytes |
| `05` | Start Linear Address | 4 bytes |

`CC` makes every byte of the record sum to zero in the low byte.

## References

- [Intel, _Hexadecimal Object File Format Specification_, Rev A, 1988](https://people.ece.cornell.edu/land/courses/ece4760/FinalProjects/s2012/ads264_mws228/Final%20Report/Final%20Report/Intel%20HEX%20Standard.pdf)
  -> Intel no longer publishes it, so every copy is an unofficial mirror.
- [srec_intel(5)](https://manpages.ubuntu.com/manpages/bionic/man5/srec_intel.5.html)
  -> readable field-by-field description, derived from the spec above.
- [Intel HEX on Wikipedia](https://en.wikipedia.org/wiki/Intel_HEX) 
  -> history and tool conventions.

## Tool name

> `ixv` ==> **i**ntel he**x** **v**erifier


#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
