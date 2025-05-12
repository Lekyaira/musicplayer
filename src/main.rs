mod gui;
mod player;
mod cli;
mod utils;

use anyhow::Result;
use clap::Parser;
use glob::glob;
use std::path::PathBuf;
use utils::is_audio_file;

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
            if is_audio_file(&path) {
                files.push(path);
            } else {
                eprintln!("Skipping non-audio file: {}", pattern);
            }
            continue;
        }
        
        // Try as a glob pattern
        match glob(&pattern) {
            Ok(entries) => {
                let mut matched = false;
                let mut audio_matched = false;
                
                for path in entries.flatten() {
                    if path.is_file() {
                        matched = true;
                        if is_audio_file(&path) {
                            audio_matched = true;
                            files.push(path);
                        } // Silently skip non-audio files from globs
                    }
                }
                
                if !matched {
                    eprintln!("No files matched pattern: {}", pattern);
                } else if !audio_matched {
                    eprintln!("Pattern '{}' matched files, but none were audio files", pattern);
                }
            },
            Err(_) => {
                eprintln!("Invalid pattern: {}", pattern);
            }
        }
    }
    
    if files.is_empty() {
        eprintln!("No audio files found in the provided patterns");
    } else {
        println!("Found {} audio files", files.len());
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
