use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "The Magnificent Git Wiper")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Remove blobs larger than a specified size
    StripBlobs {
        #[arg(long)]
        size: u64, // In butes for now
        #[arg(long)]
        repo_path: String,
    },
    /// Delete files matching a pattern
    DeleteFiles {
        #[arg(long)]
        pattern: String,
        #[arg(long)]
        repo_path: String,
    },
    /// Replace test in files throughout history
    ReplaceText {
        #[arg(long)]
        replacements_file: String,
        #[arg(long)]
        repo_path: String,
    },
}
