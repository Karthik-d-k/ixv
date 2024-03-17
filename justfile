# List commands
default: clear
    @just --list

# Clear screen
clear:
	clear

# Debug build
dbuild: clear
    cargo build

# Release build
rbuild: clear
    cargo build --release

# Debug build run
drun args='': clear
    cargo run -- {{args}}

# Release build run
rrun args='': clear
    cargo run --release -- {{args}}

# Print library size
size: clear dbuild rbuild
    @ls -sh ./target/debug/ixv
    @ls -sh ./target/release/ixv

# Run Tests
test: clear
	cargo test --release -- --nocapture

# Clean target
clean: clear
    cargo clean

# Git
git: clear
    git status
    git diff
