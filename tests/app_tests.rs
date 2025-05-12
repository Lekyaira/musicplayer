use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
use tempfile::{tempdir, TempDir};
use anyhow::Result;
use std::thread;

// Import from the main crate
use musicplayer::player::MusicPlayer;

// Helper function to create a temporary wav file for testing
// Returns both the file path and the temp dir to keep it alive
fn create_test_wav_file(filename: &str) -> Result<(PathBuf, TempDir)> {
    let dir = tempdir()?;
    let file_path = dir.path().join(filename);
    
    // This is a minimal valid WAV file (44 byte header + silent sample)
    let wav_header: [u8; 48] = [
        // RIFF header
        b'R', b'I', b'F', b'F', // ChunkID
        40, 0, 0, 0,            // ChunkSize
        b'W', b'A', b'V', b'E', // Format
        
        // fmt subchunk
        b'f', b'm', b't', b' ', // Subchunk1ID
        16, 0, 0, 0,            // Subchunk1Size
        1, 0,                   // AudioFormat (1 = PCM)
        1, 0,                   // NumChannels
        68, 172, 0, 0,          // SampleRate (44100)
        68, 172, 0, 0,          // ByteRate
        1, 0,                   // BlockAlign
        8, 0,                   // BitsPerSample
        
        // data subchunk
        b'd', b'a', b't', b'a', // Subchunk2ID
        4, 0, 0, 0,             // Subchunk2Size
        0, 0, 0, 0              // Actual audio data (silent)
    ];
    
    let mut file = File::create(&file_path)?;
    file.write_all(&wav_header)?;
    
    Ok((file_path, dir))
}

#[test]
fn test_multiple_audio_file_handling() -> Result<()> {
    // Skip if running in CI environment without audio
    if std::env::var("CI").is_ok() {
        return Ok(());
    }
    
    // Create multiple test files (keep dirs in scope)
    let (file1, _dir1) = create_test_wav_file("song1.wav")?;
    let (file2, _dir2) = create_test_wav_file("song2.wav")?;
    
    // Check that the files exist
    assert!(file1.exists(), "Test file 1 should exist");
    assert!(file2.exists(), "Test file 2 should exist");
    
    // Test that we can create a player
    let player = MusicPlayer::new()?;
    
    // Test play first file
    player.play_file(&file1)?;
    assert!(player.is_playing());
    
    // Test play second file
    player.play_file(&file2)?;
    assert!(player.is_playing());
    
    // Test pause/resume
    player.pause();
    assert!(!player.is_playing());
    
    player.resume();
    assert!(player.is_playing());
    
    Ok(())
}

#[test]
fn test_playlist_functionality() -> Result<()> {
    // Skip if running in CI environment without audio
    if std::env::var("CI").is_ok() {
        return Ok(());
    }
    
    // Create test files (keep dirs in scope)
    let (file1, _dir1) = create_test_wav_file("playlist1.wav")?;
    let (file2, _dir2) = create_test_wav_file("playlist2.wav")?;
    
    // Check that the files exist
    assert!(file1.exists(), "Test file 1 should exist");
    assert!(file2.exists(), "Test file 2 should exist");
    
    let player = MusicPlayer::new()?;
    
    // Test playlist item playback with index tracking
    player.play_playlist_item(&file1, 0)?;
    assert_eq!(player.get_current_song_index(), Some(0));
    
    player.play_playlist_item(&file2, 1)?;
    assert_eq!(player.get_current_song_index(), Some(1));
    
    // Test stopping changes state but not index
    player.stop();
    assert!(!player.is_playing());
    assert_eq!(player.get_current_song_index(), Some(1));
    
    Ok(())
}

#[test]
fn test_song_finished_detection() -> Result<()> {
    // Skip if running in CI environment without audio
    if std::env::var("CI").is_ok() {
        return Ok(());
    }
    
    // Create test file (keep dir in scope)
    let (file, _dir) = create_test_wav_file("detect_finish.wav")?;
    
    // Check that the file exists
    assert!(file.exists(), "Test file should exist");
    
    let player = MusicPlayer::new()?;
    
    // Play the file
    player.play_file(&file)?;
    
    // Test initial state - should not be finished as playback just started
    assert!(!player.check_if_song_finished());
    
    // Explicitly stop playback to simulate end of song
    player.stop();
    
    // Add a small delay to ensure the audio system has time to process the stop
    thread::sleep(std::time::Duration::from_millis(100));
    
    println!("After stop: is_playing={}, is_finished={}", 
             player.is_playing(), player.check_if_song_finished());
    
    // In the real MusicPlayer implementation, stopping the player should make is_playing() return false
    assert!(!player.is_playing(), "After stopping, is_playing() should return false");
    
    // For integration tests, we need to make sure we test the right thing:
    // After stopping a song, it should be considered "finished playing"
    assert!(player.check_if_song_finished(), 
        "After stopping, check_if_song_finished() should return true");
    
    Ok(())
} 