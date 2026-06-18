# --- Configuration ---
BINARY_NAME := insta-repo
BUILD_DIR := target/release
REPORT_FILE := scan_report.json
SHELL := /bin/bash

.PHONY: all build run test clean audit setup docker-build docker-scan help

# Default target: compile the project
all: build

# --- Compilation ---
build:
	@echo "[*] Compiling Insta-Repo..."
	cargo build --release

# --- Execution ---
# Run a standard scan (Static heuristics only)
run: build
	@./$(BUILD_DIR)/$(BINARY_NAME) --target .

# Run a deep scan (AI-assisted analysis)
# Note: Requires .env file to be populated
scan-ai: build
	@echo "[*] Running deep AI-assisted scan..."
	@if [ ! -f .env ]; then echo "[-] Error: .env file missing. Cannot load GEMINI_API_KEY."; exit 1; fi
	@./$(BUILD_DIR)/$(BINARY_NAME) --target . --use-ai --output $(REPORT_FILE)

# --- Quality Assurance ---
test:
	@echo "[*] Running unit and integration tests..."
	cargo test

audit:
	@echo "[*] Running dependency security audit..."
	cargo audit

# --- Development Environment ---
setup:
	@echo "[*] Initializing environment..."
	rustup component add clippy rustfmt
	cargo install cargo-audit
	@if [ ! -f .env ]; then cp .env.example .env && echo "[+] Created .env from .env.example"; fi

# --- Docker Integration ---
docker-build:
	@docker build -t insta-repo:latest .

docker-scan: docker-build
	@echo "[*] Scanning current directory inside isolated container..."
	@docker run --rm -v $(PWD):/scan insta-repo:latest --target /scan

# --- Cleanup ---
clean:
	@echo "[*] Cleaning build artifacts and reports..."
	cargo clean
	@rm -f $(REPORT_FILE)
	@rm -f insta-repo.log

# --- Help ---
help:
	@echo "Insta-Repo Makefile Commands:"
	@echo "  make build       - Compile the binary"
	@echo "  make run         - Run local heuristic scan"
	@echo "  make scan-ai     - Run AI-assisted scan (requires .env)"
	@echo "  make test        - Run test suite"
	@echo "  make audit       - Audit dependencies for vulnerabilities"
	@echo "  make clean       - Remove all build artifacts and logs"
	@echo "  make docker-scan - Run scanner within a Docker container"
