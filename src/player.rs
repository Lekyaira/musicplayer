use anyhow::Result;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub struct MusicPlayer {
    sink: Sink,
    _stream: OutputStream,
    _stream_handle: rodio::OutputStreamHandle,
}

impl MusicPlayer {
    pub fn new() -> Result<Self> {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;
        
        Ok(Self {
            sink,
            _stream,
            _stream_handle: stream_handle,
        })
    }

    pub fn play_file(&self, path: &Path) -> Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let decoder = Decoder::new(reader)?;
        
        self.sink.append(decoder);
        self.sink.play();
        
        Ok(())
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
        !self.sink.is_paused()
    }
} 