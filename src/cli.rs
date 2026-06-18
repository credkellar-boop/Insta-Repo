use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "Insta-Repo: Advanced Security Scanner")]
pub struct Cli {
    #[arg(short, long, default_value = ".")]
    pub target: PathBuf,

    #[arg(long)]
    pub use_ai: bool,

    #[arg(short, long)]
    pub output: Option<PathBuf>,
}
