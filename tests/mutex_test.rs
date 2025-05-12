use anyhow::Result;
use std::sync::{Arc, Mutex};

// A simplified version of the player to test mutex behavior
struct SimplifiedPlayer {
    current_song_index: Arc<Mutex<Option<usize>>>,
}

impl SimplifiedPlayer {
    fn new() -> Self {
        Self {
            current_song_index: Arc::new(Mutex::new(None)),
        }
    }
    
    fn set_index(&self, index: usize) -> Result<()> {
        if let Ok(mut current_index) = self.current_song_index.lock() {
            *current_index = Some(index);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to lock mutex"))
        }
    }
    
    fn get_index(&self) -> Option<usize> {
        if let Ok(guard) = self.current_song_index.lock() {
            *guard
        } else {
            None
        }
    }
}

#[test]
fn test_mutex_state() {
    let player = SimplifiedPlayer::new();
    assert_eq!(player.get_index(), None);
    
    let result = player.set_index(5);
    assert!(result.is_ok());
    
    let index = player.get_index();
    println!("Retrieved index: {:?}", index);
    assert_eq!(index, Some(5));
} 