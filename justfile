# List commands
default:
    @just --list

# Debug build
dbuild:
    cargo build

# Release build
rbuild:
    cargo build --release

# Debug build run
drun args='':
    cargo run -- {{args}}

# Release build run
rrun args='':
    cargo run --release -- {{args}}

# Print library size
size: dbuild rbuild
    @ls -sh ./target/debug/ixv
    @ls -sh ./target/release/ixv

# Run Tests
test:
	cargo test --release -- --nocapture

# Clean target
clean:
    cargo clean

# Git
git:
    git status
    git diff
