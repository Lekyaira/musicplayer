use anyhow::Result;
use std::path::PathBuf;
use crate::player::MusicPlayer;

pub fn run(path: Option<String>) -> Result<()> {
    let player = MusicPlayer::new()?;
    
    if let Some(path) = path {
        let path = PathBuf::from(path);
        if path.is_file() {
            println!("Playing: {}", path.display());
            player.play_file(&path)?;
            // Keep the program running while the music plays
            std::thread::sleep(std::time::Duration::from_secs(1));
            while player.is_playing() {
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        } else {
            println!("Error: Path is not a file");
        }
    } else {
        println!("Error: No file path provided");
    }

    Ok(())
} 