use std::process;

fn main() {
    if let Err(e) = ixv::run() {
        eprintln!("[ixv error]: {}", e);

        process::exit(1);
    }
}
