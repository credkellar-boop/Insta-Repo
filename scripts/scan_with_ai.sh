#!/bin/bash
set -e

# Ensure we have the .env loaded for the API keys
if [ -f .env ]; then
    export $(cat .env | xargs)
else
    echo "[-] No .env file found. AI analysis will fail if GEMINI_API_KEY is missing in the system environment."
fi

# Build and run with the AI flag against the current directory
echo "[*] Compiling latest binary..."
cargo build --release

echo "[*] Executing deep scan with AI reasoning engine..."
./target/release/insta-repo --target . --use-ai
