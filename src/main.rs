mod gui;
mod player;
mod cli;

use anyhow::Result;
use clap::Parser;
use glob::glob;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run in CLI mode
    #[arg(short, long)]
    cli: bool,

    /// Path to music files or glob patterns (e.g., "*.mp3")
    #[arg(value_name = "FILES")]
    files: Vec<String>,
}

fn expand_glob_patterns(patterns: Vec<String>) -> Vec<PathBuf> {
    let mut files = Vec::new();
    
    for pattern in patterns {
        // Check if it's a direct file path
        let path = PathBuf::from(&pattern);
        if path.is_file() {
            files.push(path);
            continue;
        }
        
        // Try as a glob pattern
        match glob(&pattern) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(path) = entry {
                        if path.is_file() {
                            files.push(path);
                        }
                    }
                }
            },
            Err(_) => {
                // Ignore invalid patterns
                eprintln!("Invalid pattern: {}", pattern);
            }
        }
    }
    
    files
}

fn main() -> Result<()> {
    let args = Args::parse();
    let file_paths = expand_glob_patterns(args.files);

    if args.cli {
        if let Some(first_file) = file_paths.first() {
            if let Some(path_str) = first_file.to_str() {
                cli::run(Some(path_str.to_string()))?;
            } else {
                cli::run(None)?;
            }
        } else {
            cli::run(None)?;
        }
    } else {
        gui::run(file_paths)?;
    }

    Ok(())
}
