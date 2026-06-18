# Advanced Usage Guide

## Adding Custom Heuristics
You can add new regex patterns in `rules/signatures.yaml`. The `pattern` field uses standard Rust regex syntax.

## API Integration
Insta-Repo supports Gemini 1.5 Pro. Ensure `GEMINI_API_KEY` is set in your environment.
The scanner automatically skips files > 5MB to optimize token usage.

## eBPF (Roadmap)
Dynamic analysis via eBPF probes is currently under development. Monitor `src/ebpf_monitor.rs` for updates.
