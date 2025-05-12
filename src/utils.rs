use std::path::Path;
use std::collections::HashSet;
use lazy_static::lazy_static;

lazy_static! {
    /// A set of supported audio file extensions
    static ref SUPPORTED_AUDIO_EXTENSIONS: HashSet<&'static str> = {
        let mut extensions = HashSet::new();
        extensions.insert("mp3");
        extensions.insert("wav");
        extensions.insert("ogg");
        extensions.insert("flac");
        extensions.insert("aac");
        extensions.insert("m4a");
        extensions.insert("opus");
        extensions.insert("wma");
        extensions
    };
}

/// Check if a file is an audio file based on its extension
pub fn is_audio_file<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();
    
    if let Some(extension) = path.extension() {
        if let Some(ext_str) = extension.to_str() {
            return SUPPORTED_AUDIO_EXTENSIONS.contains(ext_str.to_lowercase().as_str());
        }
    }
    
    false
}

/// Get a slice of supported audio extensions for file dialogs
pub fn get_supported_extensions() -> Vec<&'static str> {
    SUPPORTED_AUDIO_EXTENSIONS.iter().cloned().collect()
} 