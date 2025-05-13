mod gui;
mod player;
mod utils;

use anyhow::Result;
use clap::Parser;
use glob::glob;
use std::path::PathBuf;
use utils::is_audio_file;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to music files or glob patterns (e.g., "*.mp3")
    #[arg(value_name = "FILES")]
    files: Vec<String>,

    /// When true, the app was launched via "Open with" from the OS
    #[arg(long, hide = true)]
    opened_with: bool,
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
    
    // Detect if app was launched via OS file association
    // On macOS, if the app is launched via "Open with", the first argument will be -psn_*
    // This is macOS-specific process serial number
    let is_macos_file_open = std::env::args().any(|arg| arg.starts_with("-psn_"));
    
    // Get files from command-line args
    let mut file_paths = expand_glob_patterns(args.files);
    
    // On Windows/Linux, the files are passed directly as arguments
    // On macOS, we need to check for AppleEvents (via eframe's integration)
    // If no files found yet and we're launched via file association,
    // eframe will handle it via context.dropped_files in the app
    
    // Launch the GUI with the files
    gui::run(file_paths, is_macos_file_open || args.opened_with)
}
