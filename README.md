1781744900198.png
# Insta-Repo 🛡️

![GitHub stars](https://img.shields.io/github/stars/credkellar-boop/Insta-Repo?style=social)
![GitHub forks](https://img.shields.io/github/forks/credkellar-boop/Insta-Repo?style=social)

**Languages & Frameworks**
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Bash](https://img.shields.io/badge/bash-%23121011.svg?style=for-the-badge&logo=gnu-bash&logoColor=white)
![Solidity](https://img.shields.io/badge/Solidity-%23363636.svg?style=for-the-badge&logo=solidity&logoColor=white)

**Infrastructure & DevOps**
![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)
![GitHub Actions](https://img.shields.io/badge/github%20actions-%232671E5.svg?style=for-the-badge&logo=githubactions&logoColor=white)
![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)

**Cybersecurity & Forensics**
![eBPF](https://img.shields.io/badge/eBPF-Runtime_Monitor-FF4081?style=for-the-badge)
![Google Gemini](https://img.shields.io/badge/Gemini_AI-Semantic_Analysis-8E75B2?style=for-the-badge&logo=google)
![Security Audit](https://img.shields.io/badge/Security-DevSecOps-red?style=for-the-badge&logo=owasp)

## 📖 What is this about?
**Insta-Repo** is a highly specialized, fast, and recursive security scanner written in Rust. It is engineered to analyze entire project directories for security red flags, supply-chain attacks, hidden malicious payloads, and vulnerable smart contracts. It serves as a frontline automated defense mechanism, bridging lightning-fast static regex parsing with deep semantic reasoning powered by AI and runtime monitoring via eBPF.

## ⚡ Why this is cool in 6 seconds
* **Blazing Fast:** Built in Rust, aggressively recursing through directories while intelligently bypassing massive binaries (>5MB) to optimize memory overhead.
* **AI-Powered Threat Hunting:** Dispatches heavily obfuscated scripts or metamorphic payloads to a reasoning model (Gemini 1.5 Pro / Flash-Lite) to decode the true intent behind the code.
* **Web3 & Crypto Forensics:** Actively hunts for targeted attacks on `.ethereum/keystore` files, stolen SSH keys, and malicious Web3 patterns (e.g., unauthorized `selfdestruct` in Solidity).
* **Supply-Chain Defense:** Instantly flags suspicious build-time execution, such as rogue `Command::new()` spawns in `build.rs` or `Makefiles`.

## 🛡️ Cybersecurity Threat Model & Capabilities
Insta-Repo is built around a rigorous threat model (`docs/THREAT_MODEL.md`) designed to catch both static and behavioral indicators of compromise (IoCs):
* **Static Heuristics Engine:** Parses source code against custom YARA-style signatures (`signatures.yaml`, `smart_contract.yaml`) to detect base64 obfuscation, hardcoded wallet targets, and `curl | bash` execution paths.
* **Smart Contract Auditing:** Dedicated rulesets to identify backdoor contracts, hidden miners, and wallet drainers before deployment.
* **eBPF Runtime Monitoring:** (`src/ebpf_monitor.rs`) Hooks into the kernel layer to monitor unexpected syscalls and network connections during dependency resolution.

## 🏗️ Infrastructure & Deployment Architecture
Built for seamless integration into modern DevSecOps pipelines:
* **Containerized Execution:** Fully isolated scanning environment via Docker. `make docker-scan` ensures the scanner runs without exposing the host OS to potentially active payloads.
* **CI/CD Native:** Push reports directly to `stdout` or serialize them to JSON (`src/reporter.rs`) for automated ingestion into GitHub Actions or GitLab CI.
* **Automated Audits:** Integrated Dependabot configuration (`.github/dependabot.yml`) and local auditing scripts (`scripts/audit.sh`) to maintain the integrity of the scanner itself.

## 🚀 How to Install & Run

### Prerequisites
* **Rust & Cargo** (for local compilation)
* **Docker** (for isolated infrastructure scanning)
* **Gemini API Key** (for Semantic AI Analysis)

### Quick Start
1. **Clone the repository:**
   ```bash
   git clone [https://github.com/credkellar-boop/Insta-Repo.git](https://github.com/credkellar-boop/Insta-Repo.git)
   cd Insta-Repo
