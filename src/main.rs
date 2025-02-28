mod cli;
mod file_ops;
mod git_ops;

use clap::Parser;
use cli::{Cli, Commands};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::StripBlobs { size, repo_path } => {
            let repo = git_ops::clone_repo(&repo_path, "temp_repo")?;
            let large_blobs = file_ops::find_large_blobs(&repo, size);
            git_ops::rewrite_history(&repo, large_blobs)?;
            println!("Stripped blobs larger than {} bytes", size);
        }
        Commands::DeleteFiles { pattern, repo_path } => {
            let repo = git_ops::clone_repo(&repo_path, "temp_repo")?;
            let files = file_ops::match_files(&repo_path, &pattern);
            git_ops::filter_branch(&repo, files)?;
            println!("Deleted files matching pattern: {}", pattern);
        }
        Commands::ReplaceText {
            replacements_file,
            repo_path,
        } => {
            println!("Text replacement not yet implemented");
        }
    }
    Ok(())
}
