use anyhow::Result;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct MusicPlayer {
    sink: Sink,
    _stream: OutputStream,
    _stream_handle: rodio::OutputStreamHandle,
    current_song_index: Arc<Mutex<Option<usize>>>,
    is_song_finished: Arc<Mutex<bool>>,
}

impl MusicPlayer {
    pub fn new() -> Result<Self> {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;
        
        Ok(Self {
            sink,
            _stream,
            _stream_handle: stream_handle,
            current_song_index: Arc::new(Mutex::new(None)),
            is_song_finished: Arc::new(Mutex::new(false)),
        })
    }

    pub fn play_file(&self, path: &Path) -> Result<()> {
        self.sink.stop();
        
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let decoder = Decoder::new(reader)?;
        
        self.sink.append(decoder);
        self.sink.play();
        
        Ok(())
    }

    pub fn play_playlist_item(&self, path: &Path, index: usize) -> Result<()> {
        self.play_file(path)?;
        
        // Store the current playing index
        if let Ok(mut current_index) = self.current_song_index.lock() {
            *current_index = Some(index);
        }
        
        // Reset song finished flag
        if let Ok(mut flag) = self.is_song_finished.lock() {
            *flag = false;
        }
        
        Ok(())
    }
    
    pub fn check_if_song_finished(&self) -> bool {
        let empty = self.sink.empty();
        let paused = self.sink.is_paused();
        
        let song_completed = empty && !paused;
        
        if song_completed {
            if let Ok(mut flag) = self.is_song_finished.lock() {
                *flag = true;
            }
        }
        
        song_completed
    }
    
    pub fn get_current_song_index(&self) -> Option<usize> {
        self.current_song_index.lock().ok().and_then(|guard| *guard)
    }

    pub fn pause(&self) {
        self.sink.pause();
    }

    pub fn resume(&self) {
        self.sink.play();
    }

    pub fn stop(&self) {
        self.sink.stop();
    }

    pub fn is_playing(&self) -> bool {
        !self.sink.is_paused() && !self.sink.empty()
    }
} 