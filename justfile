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

# Release build
run: clear
    cargo run --release --quiet

# Print library size
size: clear dbuild rbuild
    @ls -sh ./target/debug/ixv
    @ls -sh ./target/release/ixv

# Clean target
clean: clear
    cargo clean

# Git
git: clear
    git status
    git diff
