use anyhow::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

const APP_NAME: &str = "musicplayer";
const ORG_NAME: &str = "musicplayer";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub filename: String,
    pub volume: f32,
    // Add more config options here in the future
}

impl Default for Config {
    fn default() -> Self {
        Self {
            filename: "config.toml".to_string(),
            volume: 0.5,
        }
    }
}

/// Gets the config directory, creating it if it doesn't exist
fn get_config_dir() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("", ORG_NAME, APP_NAME)
        .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;
    
    let config_dir = proj_dirs.config_dir();
    if !config_dir.exists() {
        fs::create_dir_all(config_dir)?;
    }
    
    Ok(config_dir.to_path_buf())
}

/// Gets the config file path
fn get_config_file_path() -> Result<PathBuf> {
    let config_dir = get_config_dir()?;
    Ok(config_dir.join("config.toml"))
}

/// Loads the configuration from disk, or creates a default one if not found
pub fn load_config() -> Result<Config> {
    let config_path = get_config_file_path()?;
    
    if !config_path.exists() {
        let default_config = Config::default();
        save_config(&default_config)?;
        return Ok(default_config);
    }
    
    let mut file = File::open(config_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}

/// Saves the configuration to disk
pub fn save_config(config: &Config) -> Result<()> {
    let config_path = get_config_file_path()?;
    let serialized = toml::to_string_pretty(config)?;
    
    let mut file = File::create(config_path)?;
    file.write_all(serialized.as_bytes())?;
    
    Ok(())
}

/// Returns a user-friendly description of where the config file is stored
pub fn get_config_location_description() -> String {
    if let Ok(path) = get_config_file_path() {
        format!("Configuration is stored at: {}", path.display())
    } else {
        "Could not determine configuration location".to_string()
    }
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default_values() {
        // Test that default config has the expected values
        let default_config = Config::default();
        assert_eq!(default_config.volume, 0.5);
        assert_eq!(default_config.filename, "config.toml");
    }

    #[test]
    fn test_config_save_and_load() {
        // Initialize a new config
        let test_config = Config {
            filename: "test.toml".to_string(),
            volume: 0.75,
        };

        // Save the config to disk
        save_config(&test_config).expect("Failed to save config");

        // Load the config from disk
        let loaded_config = load_config().expect("Failed to load config");

        // Test!
        assert_eq!(loaded_config.volume, 0.75);
        assert_eq!(loaded_config.filename, "test.toml");
    }
    
    #[test]
    fn test_get_config_location_description() {
        // Get the config location description
        let location_desc = get_config_location_description();

        // Get the OS 
        let os = std::env::consts::OS;

        // Get the expected OS path 
        let expected_path = match os {
            "windows" => format!("C:\\Users\\{}\\AppData\\Roaming\\musicplayer\\config.toml", std::env::var("USERNAME").unwrap()),
            "linux" => format!("/home/{}/.config/musicplayer/config.toml", std::env::var("USER").unwrap()),
            "macos" => format!("/Users/{}/Library/Application Support/musicplayer.musicplayer/config.toml", std::env::var("USER").unwrap()),
            _ => panic!("Unsupported OS"),
        };

        // Test!
        assert_eq!(location_desc, format!("Configuration is stored at: {}", expected_path));

    }
}
