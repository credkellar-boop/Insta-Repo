# Insta-Repo

A fast, recursive, heuristic-based security scanner written in Rust. It analyzes entire project directories for security red flags, supply-chain attacks, and hidden payloads, helping you audit code before execution.

## Features
- **Deep Scanning:** Recursively traverses directories while safely ignoring massive binaries (>5MB).
- **Supply-Chain Defense:** Flags suspicious `Command::new()` spawns in `build.rs` or Makefiles.
- **Threat Detection:** Identifies `curl | bash` stagers, raw hex shellcode, base64 obfuscation, and dynamic `eval()` execution.
- **Crypto-Stealer Signatures:** Detects hardcoded targets for `.ethereum/keystore` or SSH keys.

## Installation
Ensure you have Rust installed, then build from source:

```bash
git clone [https://github.com/credkellar-boop/insta-repo.git](https://github.com/credkellar-boop/insta-repo.git)
cd insta-repo
cargo build --release
