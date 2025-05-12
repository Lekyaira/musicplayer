use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;
use anyhow::Result;

// A mock player implementation that simulates the real player without audio
#[derive(Clone)]
struct MockPlayer {
    current_index: Arc<Mutex<Option<usize>>>,
    is_playing: Arc<Mutex<bool>>,
    is_finished: Arc<Mutex<bool>>,
}

impl MockPlayer {
    fn new() -> Self {
        Self {
            current_index: Arc::new(Mutex::new(None)),
            is_playing: Arc::new(Mutex::new(false)),
            is_finished: Arc::new(Mutex::new(false)),
        }
    }
    
    fn play(&self, _path: &Path, index: usize) -> Result<()> {
        if let Ok(mut current) = self.current_index.lock() {
            *current = Some(index);
        } else {
            return Err(anyhow::anyhow!("Failed to lock current index"));
        }
        
        if let Ok(mut playing) = self.is_playing.lock() {
            *playing = true;
        } else {
            return Err(anyhow::anyhow!("Failed to lock playing state"));
        }
        
        if let Ok(mut finished) = self.is_finished.lock() {
            *finished = false;
        } else {
            return Err(anyhow::anyhow!("Failed to lock finished state"));
        }
        
        Ok(())
    }
    
    fn stop(&self) {
        if let Ok(mut playing) = self.is_playing.lock() {
            *playing = false;
        }
        
        if let Ok(mut finished) = self.is_finished.lock() {
            *finished = true;
        }
    }
    
    fn pause(&self) {
        if let Ok(mut playing) = self.is_playing.lock() {
            *playing = false;
        }
    }
    
    fn resume(&self) {
        if let Ok(mut playing) = self.is_playing.lock() {
            *playing = true;
        }
    }
    
    fn is_playing(&self) -> bool {
        // Match the real player's behavior:
        // - Not playing if paused
        // - Not playing if finished
        if let (Ok(playing), Ok(finished)) = (self.is_playing.lock(), self.is_finished.lock()) {
            return *playing && !*finished;
        }
        false
    }
    
    fn is_finished(&self) -> bool {
        if let Ok(finished) = self.is_finished.lock() {
            *finished
        } else {
            false
        }
    }
    
    fn get_current_index(&self) -> Option<usize> {
        if let Ok(current) = self.current_index.lock() {
            *current
        } else {
            None
        }
    }
    
    // Simulate playing a song to completion
    fn simulate_playback_complete(&self) {
        // First stop the player
        self.stop();
        
        // Then set finished flag
        if let Ok(mut finished) = self.is_finished.lock() {
            *finished = true;
        }
    }
}

#[test]
fn test_mock_player_basic() -> Result<()> {
    let player = MockPlayer::new();
    
    // Initial state
    assert!(!player.is_playing());
    assert!(!player.is_finished());
    assert_eq!(player.get_current_index(), None);
    
    // Play a track
    let path = PathBuf::from("/mock/track1.mp3");
    player.play(&path, 0)?;
    
    // Check state after playing
    assert!(player.is_playing());
    assert!(!player.is_finished());
    assert_eq!(player.get_current_index(), Some(0));
    
    // Test pause/resume
    player.pause();
    assert!(!player.is_playing());
    
    player.resume();
    assert!(player.is_playing());
    
    // Test stopping
    player.stop();
    assert!(!player.is_playing());
    assert!(player.is_finished());
    
    Ok(())
}

#[test]
fn test_mock_player_playlist() -> Result<()> {
    let player = MockPlayer::new();
    
    // Setup mock playlist - use array instead of vec! to avoid "useless vec!" warning
    let tracks = [
        PathBuf::from("/mock/track1.mp3"),
        PathBuf::from("/mock/track2.mp3"),
        PathBuf::from("/mock/track3.mp3"),
    ];
    
    // Play first track
    player.play(&tracks[0], 0)?;
    assert_eq!(player.get_current_index(), Some(0));
    
    // Simulate track completion and move to next
    player.simulate_playback_complete();
    assert!(player.is_finished());
    
    // Play next track
    player.play(&tracks[1], 1)?;
    assert_eq!(player.get_current_index(), Some(1));
    assert!(!player.is_finished());
    
    Ok(())
}

#[test]
fn test_mock_player_threading() -> Result<()> {
    let player = MockPlayer::new();
    
    // Setup mock track
    let track = PathBuf::from("/mock/track1.mp3");
    
    // Play first track
    player.play(&track, 0)?;
    assert!(player.is_playing());
    
    // Simulate song ending in another thread
    let player_clone = player.clone();
    let handle = thread::spawn(move || {
        // Simulate playback duration
        thread::sleep(Duration::from_millis(100));
        
        // End playback
        player_clone.simulate_playback_complete();
    });
    
    // Wait for thread to complete
    handle.join().unwrap();
    
    // Playback should now be finished
    assert!(!player.is_playing());
    assert!(player.is_finished());
    
    Ok(())
} 