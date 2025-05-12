use anyhow::Result;
use eframe::{ egui, egui::ViewportBuilder, NativeOptions };
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use crate::player::MusicPlayer;

struct MusicPlayerApp {
    player: Arc<Mutex<MusicPlayer>>,
    current_file: Option<PathBuf>,
    started_playing: bool,
    playlist: Vec<PathBuf>,
    current_playlist_index: Option<usize>,
    selected_song_index: Option<usize>,
    is_playing: bool,
}

impl MusicPlayerApp {
    fn new(cc: &eframe::CreationContext<'_>, path: Option<String>) -> Self {
        let mut file: Option<PathBuf> = None;
        let mut started_playing: bool = false;
        let mut playlist = Vec::new();
        
        if let Some(path) = path {
            let path = PathBuf::from(path);
            if path.is_file() {
                file = Some(path.clone());
                playlist.push(path);
                started_playing = true;
            }
        }

        Self {
            player: Arc::new(Mutex::new(MusicPlayer::new().unwrap())),
            current_file: file,
            started_playing,
            playlist,
            current_playlist_index: None,
            selected_song_index: None,
            is_playing: false,
        }
    }
    
    fn play_current_song(&mut self) {
        if let Some(index) = self.current_playlist_index {
            if index < self.playlist.len() {
                let path = &self.playlist[index];
                self.current_file = Some(path.clone());
                if let Ok(player) = self.player.lock() {
                    let _ = player.play_playlist_item(path, index);
                    self.is_playing = true;
                }
            }
        }
    }
    
    fn play_next_song(&mut self) {
        let next_index = if let Some(current) = self.current_playlist_index {
            if current + 1 < self.playlist.len() {
                Some(current + 1)
            } else {
                None // End of playlist
            }
        } else if !self.playlist.is_empty() {
            Some(0) // Start of playlist
        } else {
            None // Empty playlist
        };
        
        self.current_playlist_index = next_index;
        if next_index.is_some() {
            self.play_current_song();
        } else {
            self.is_playing = false;
        }
    }
    
    fn add_to_playlist(&mut self) {
        if let Some(paths) = rfd::FileDialog::new()
            .add_filter("Audio Files", &["mp3", "wav", "ogg", "flac"])
            .pick_files()
        {
            for path in paths {
                self.playlist.push(path);
            }
            
            // If no song is playing, start with the first added song
            if self.current_playlist_index.is_none() && !self.playlist.is_empty() {
                self.current_playlist_index = Some(0);
                self.play_current_song();
            }
        }
    }
    
    fn remove_from_playlist(&mut self) {
        if let Some(index) = self.selected_song_index {
            if index < self.playlist.len() {
                // If the currently playing song is removed, stop playback
                if Some(index) == self.current_playlist_index {
                    if let Ok(player) = self.player.lock() {
                        player.stop();
                    }
                    self.is_playing = false;
                }
                
                // Update current playlist index if needed
                if let Some(current) = self.current_playlist_index {
                    if index < current {
                        self.current_playlist_index = Some(current - 1);
                    } else if index == current && current == self.playlist.len() - 1 {
                        self.current_playlist_index = if current > 0 { Some(current - 1) } else { None };
                    }
                }
                
                self.playlist.remove(index);
                self.selected_song_index = None;
            }
        }
    }
    
    fn move_up_in_playlist(&mut self) {
        if let Some(index) = self.selected_song_index {
            if index > 0 && index < self.playlist.len() {
                self.playlist.swap(index, index - 1);
                // Update current index if it was one of the swapped items
                if let Some(current) = self.current_playlist_index {
                    if current == index {
                        self.current_playlist_index = Some(current - 1);
                    } else if current == index - 1 {
                        self.current_playlist_index = Some(current + 1);
                    }
                }
                self.selected_song_index = Some(index - 1);
            }
        }
    }
    
    fn move_down_in_playlist(&mut self) {
        if let Some(index) = self.selected_song_index {
            if index < self.playlist.len() - 1 {
                self.playlist.swap(index, index + 1);
                // Update current index if it was one of the swapped items
                if let Some(current) = self.current_playlist_index {
                    if current == index {
                        self.current_playlist_index = Some(current + 1);
                    } else if current == index + 1 {
                        self.current_playlist_index = Some(current - 1);
                    }
                }
                self.selected_song_index = Some(index + 1);
            }
        }
    }
    
    fn check_song_finished(&mut self) {
        if self.is_playing {
            let song_finished = if let Ok(player) = self.player.lock() {
                player.check_if_song_finished()
            } else {
                false
            };
            
            if song_finished {
                self.play_next_song();
            }
        }
    }
}

impl eframe::App for MusicPlayerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.started_playing {
            self.started_playing = false;
            if let Some(path) = &self.current_file {
                if let Ok(player) = self.player.lock() {
                    if self.current_playlist_index.is_none() {
                        self.current_playlist_index = Some(0);
                    }
                    let _ = player.play_playlist_item(path, self.current_playlist_index.unwrap());
                    self.is_playing = true;
                }
            }
        }
        
        // Check if current song has finished and we need to play the next one
        self.check_song_finished();
        
        // Request continuous repaint for checking song status
        ctx.request_repaint_after(std::time::Duration::from_millis(500));
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Music Player");
            
            ui.horizontal(|ui| {
                if ui.button("Add Songs").clicked() {
                    self.add_to_playlist();
                }
                
                if let Some(index) = self.selected_song_index {
                    if ui.button("Remove").clicked() {
                        self.remove_from_playlist();
                    }
                    
                    if ui.button("Move Up").clicked() {
                        self.move_up_in_playlist();
                    }
                    
                    if ui.button("Move Down").clicked() {
                        self.move_down_in_playlist();
                    }
                }
            });
            
            ui.separator();
            
            // Display playlist
            let playlist_ui = egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Playlist");
                
                for (index, path) in self.playlist.iter().enumerate() {
                    let is_selected = Some(index) == self.selected_song_index;
                    let is_playing = Some(index) == self.current_playlist_index && self.is_playing;
                    
                    let text = format!("{}. {}", index + 1, path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("Unknown"));
                    
                    let response = ui.selectable_label(is_selected, if is_playing {
                        format!("▶ {}", text)
                    } else {
                        text
                    });
                    
                    if response.clicked() {
                        self.selected_song_index = Some(index);
                    }
                    
                    if response.double_clicked() {
                        self.current_playlist_index = Some(index);
                        // We need to defer the playback to after the UI update
                        // to avoid the borrow checker error
                        self.started_playing = true;
                        self.current_file = Some(path.clone());
                    }
                }
            });
            
            ui.separator();

            // Playback controls
            ui.horizontal(|ui| {
                if self.is_playing {
                    if ui.button("⏸ Pause").clicked() {
                        if let Ok(player) = self.player.lock() {
                            player.pause();
                            self.is_playing = false;
                        }
                    }
                } else if let Some(_) = self.current_playlist_index {
                    if ui.button("▶ Play").clicked() {
                        if let Ok(player) = self.player.lock() {
                            player.resume();
                            self.is_playing = true;
                        }
                    }
                }
                
                if ui.button("⏹ Stop").clicked() {
                    if let Ok(player) = self.player.lock() {
                        player.stop();
                        self.is_playing = false;
                    }
                }
                
                if ui.button("⏭ Next").clicked() {
                    self.play_next_song();
                }
            });
            
            if let Some(path) = &self.current_file {
                ui.label(format!("Now playing: {}", path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Unknown")));
            }
        });
    }
}

pub fn run(path: Option<String>) -> Result<()> {
    let options = NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size(egui::vec2(500.0, 600.0)),
        ..Default::default()
    };
    
    if eframe::run_native(
        "Music Player",
        options,
        Box::new(|cc| Ok(Box::new(MusicPlayerApp::new(cc, path)))),
    ).is_err() {
        return Err(anyhow::anyhow!("Failed to run eframe"));
    }

    Ok(())
} 