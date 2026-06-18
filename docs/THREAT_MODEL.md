# Threat Model: Insta-Repo

## In-Scope
- Static detection of common stagers (`curl|bash`).
- Detection of hardcoded crypto-wallet paths.
- Semantic AI analysis of obfuscated code.

## Out-of-Scope
- Zero-day exploit detection.
- Kernel-level rootkit detection (eBPF roadmap).
- Dynamic analysis of non-text binaries.

## Known Bypasses
- Heavy custom encryption of shellcode (requires AI analysis).
- Steganographic payloads (hidden in images).
