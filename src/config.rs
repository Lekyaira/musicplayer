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
    pub volume: f32,
    // Add more config options here in the future
}

impl Default for Config {
    fn default() -> Self {
        Self {
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