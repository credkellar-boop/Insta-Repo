setup:
	rustup component add clippy rustfmt
	cargo install cargo-audit
	touch .env

scan-all:
	cargo run -- --target . --use-ai --output report.json

audit-deps:
	cargo audit
