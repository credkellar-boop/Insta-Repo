#!/bin/bash
set -e

echo "[*] Running Rust Formatter..."
cargo fmt --all -- --check

echo "[*] Running Clippy Linter..."
cargo clippy -- -D warnings

echo "[*] Running Unit & Integration Tests..."
cargo test

echo "[*] Running Dry-Run Scan on Self..."
cargo run --release -- --target .

echo "[+] Audit Complete. Ready to commit!"
