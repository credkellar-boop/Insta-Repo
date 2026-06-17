use clap::Parser;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the directory or repository to scan
    #[arg(short, long, default_value = ".")]
    target: PathBuf,
}

#[derive(Debug)]
struct Detection {
    file_path: String,
    threat_description: String,
}

fn main() {
    let args = Args::parse();

    if !args.target.exists() {
        eprintln!("Error: Target path {:?} does not exist.", args.target);
        std::process::exit(1);
    }

    println!("Initiating deep scan on: {}", args.target.display());
    println!("Scanning files for malware heuristics...\n");

    let detections = scan_directory(&args.target);

    println!("--- SECURITY ANALYSIS RESULTS ---");
    if detections.is_empty() {
        println!("[+] No immediate heuristic flags detected across the repository.");
    } else {
        println!("[!] WARNING: Suspicious patterns detected in the following files:");
        for det in detections {
            println!("  [File]: {}", det.file_path);
            println!("    └─ [Flag]: {}\n", det.threat_description);
        }
    }
}

fn scan_directory(target_dir: &Path) -> Vec<Detection> {
    let mut all_detections = Vec::new();

    // Compile regexes once before the loop for performance
    let heuristics = build_heuristics();

    for entry in WalkDir::new(target_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let path = entry.path();
        
        // Skip large files (> 5MB) to avoid memory bloat and skipping pure binaries
        if let Ok(metadata) = fs::metadata(path) {
            if metadata.len() > 5_000_000 {
                continue;
            }
        }

        // Try reading as UTF-8. If it fails, it's likely a binary and we skip static text analysis.
        if let Ok(content) = fs::read_to_string(path) {
            let file_name = path.to_string_lossy().to_string();
            let mut file_flags = Vec::new();

            // Run compiled heuristics
            for (re, warning) in &heuristics {
                if re.is_match(&content) {
                    file_flags.push(warning.to_string());
                }
            }

            // Flag highly suspicious file names/locations natively
            if path.to_string_lossy().contains(".git/hooks/") {
                file_flags.push("Code found in git hooks. This is a common persistence mechanism.".to_string());
            }

            for flag in file_flags {
                all_detections.push(Detection {
                    file_path: file_name.clone(),
                    threat_description: flag,
                });
            }
        }
    }

    all_detections
}

/// Builds the Regex patterns used to identify malicious intent
fn build_heuristics() -> Vec<(Regex, &'static str)> {
    vec![
        // 1. Classic Stagers & Network Fetchers
        (
            Regex::new(r"(?i)(curl|wget).*\|\s*(bash|sh|zsh|python|perl)").unwrap(),
            "Remote execution payload detected (Piped curl/wget)",
        ),
        // 2. Base64 Obfuscation Commands
        (
            Regex::new(r"(?i)base64\s*-d").unwrap(),
            "Base64 decoding command found. Often used to execute obfuscated payloads.",
        ),
        // 3. Raw Shellcode / Bytecode arrays
        (
            Regex::new(r"(\\x[0-9a-fA-F]{2}){8,}").unwrap(),
            "Raw hex shellcode/byte array pattern detected.",
        ),
        // 4. Crypto-Mining & Network Endpoints
        (
            Regex::new(r"(?i)(xmrig|stratum\+tcp|kdevr|\.systemd-login)").unwrap(),
            "Suspicious crypto-miner artifact or known malicious endpoint detected.",
        ),
        // 5. Build Script Supply Chain Abuse (Rust/C/C++)
        (
            Regex::new(r#"Command::new\("((bash|sh|wget|curl|nc))"\)"#).unwrap(),
            "Process execution targeting shell/network utilities. High risk in build.rs or Makefiles.",
        ),
        // 6. Python/JS Eval & OS Execution
        (
            Regex::new(r"(?i)(eval\(|exec\(|__import__\('os'\)\.system)").unwrap(),
            "Dynamic code execution (eval/exec) detected. Highly prone to abuse.",
        ),
        // 7. Wallet & Key Stealer Signatures
        (
            Regex::new(r"(?i)(\.ethereum/keystore|\.ssh/id_rsa|/.monad/|/wallet\.dat)").unwrap(),
            "Script references sensitive filesystem paths commonly targeted by info-stealing malware.",
        ),
    ]
}
