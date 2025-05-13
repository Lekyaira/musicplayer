use anyhow::Result;
use eframe::{ egui, egui::ViewportBuilder, NativeOptions };
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use crate::player::MusicPlayer;
use crate::utils::{is_audio_file, get_supported_extensions};

struct MusicPlayerApp {
    player: Arc<Mutex<MusicPlayer>>,
    current_file: Option<PathBuf>,
    started_playing: bool,
    playlist: Vec<PathBuf>,
    current_playlist_index: Option<usize>,
    selected_song_index: Option<usize>,
    is_playing: bool,
    volume: f32,
    song_position: Duration,
    song_duration: Option<Duration>,
    seeking: bool,
    seek_position: f32, // 0.0 to 1.0 for slider
}

impl MusicPlayerApp {
    fn new(_cc: &eframe::CreationContext<'_>, paths: Vec<PathBuf>) -> Self {
        let mut file: Option<PathBuf> = None;
        let mut started_playing: bool = false;
        let mut playlist = Vec::new();
        
        // Add all provided files to the playlist (they should already be filtered)
        for path in paths {
            if path.is_file() {
                // Use the first valid file as the initial file to play
                if file.is_none() {
                    file = Some(path.clone());
                    started_playing = true;
                }
                playlist.push(path);
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
            volume: 0.5,
            song_position: Duration::from_secs(0),
            song_duration: None,
            seeking: false,
            seek_position: 0.0,
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
        let extensions = get_supported_extensions();
        if let Some(paths) = rfd::FileDialog::new()
            .add_filter("Audio Files", &extensions)
            .pick_files()
        {
            let mut added = 0;
            
            for path in paths {
                if is_audio_file(&path) {
                    self.playlist.push(path);
                    added += 1;
                }
            }
            
            if added > 0 {
                // If no song is playing, start with the first added song
                if self.current_playlist_index.is_none() && !self.playlist.is_empty() {
                    self.current_playlist_index = Some(0);
                    self.play_current_song();
                }
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
                    self.current_playlist_index = match current {
                        // If removing the current item
                        c if c == index => {
                            if c > 0 {
                                // If not the first item, move to previous
                                Some(c - 1)
                            } else if self.playlist.len() > 1 {
                                // If first item and playlist has more items, stay at 0
                                // (which will point to the next song after removal)
                                Some(0)
                            } else {
                                // If removing the only item
                                None
                            }
                        },
                        // If removing an item before current, decrement current index
                        c if c > index => Some(c - 1),
                        // Otherwise keep the same index
                        c => Some(c),
                    };
                }
                
                // Remove the track
                self.playlist.remove(index);
                
                // Select the next track for better UX
                if !self.playlist.is_empty() {
                    if index < self.playlist.len() {
                        // If there's a next track at same position, select it
                        self.selected_song_index = Some(index);
                    } else {
                        // If we removed the last track, select the new last one
                        self.selected_song_index = Some(self.playlist.len() - 1);
                    }
                } else {
                    // No tracks left
                    self.selected_song_index = None;
                }
            }
        }
    }
    
    fn move_up_in_playlist(&mut self) {
        if let Some(index) = self.selected_song_index {
            if index > 0 && index < self.playlist.len() {
                self.playlist.swap(index, index - 1);
                // Update current index if it was one of the swapped items
                if let Some(current) = self.current_playlist_index {
                    self.current_playlist_index = match current {
                        c if c == index => Some(c - 1),
                        c if c == index - 1 => Some(c + 1),
                        c => Some(c),
                    };
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
                    self.current_playlist_index = match current {
                        c if c == index => Some(c + 1),
                        c if c == index + 1 => Some(c - 1),
                        c => Some(c),
                    };
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
    
    fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
        if let Ok(player) = self.player.lock() {
            player.set_volume(volume);
        }
    }
    
    fn update_song_position(&mut self) {
        if self.is_playing && !self.seeking {
            if let Ok(player) = self.player.lock() {
                self.song_position = player.get_current_position();
                
                // Update song duration if not set yet
                if self.song_duration.is_none() {
                    self.song_duration = player.get_song_duration();
                }
            }
        }
    }
    
    fn format_duration(duration: Duration) -> String {
        let total_seconds = duration.as_secs();
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        format!("{:02}:{:02}", minutes, seconds)
    }
    
    fn seek_to_position(&mut self, position_ratio: f32) {
        if let Some(duration) = self.song_duration {
            let position = Duration::from_secs_f32(position_ratio * duration.as_secs_f32());
            self.song_position = position;
            
            if let Ok(player) = self.player.lock() {
                if let Err(e) = player.seek_to(position) {
                    eprintln!("Error seeking: {}", e);
                }
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
                    
                    // Reset position tracking
                    self.song_position = Duration::from_secs(0);
                    self.song_duration = player.get_song_duration();
                }
            }
        }
        
        // Update song position
        self.update_song_position();
        
        // Check if current song has finished and we need to play the next one
        self.check_song_finished();
        
        // Request continuous repaint for checking song status
        ctx.request_repaint_after(std::time::Duration::from_millis(100));
        
        egui::CentralPanel::default().show(ctx, |ui| {
            // Use vertical layout to allow proper resizing
            ui.vertical(|ui| {
                // Top header section - fixed height
                ui.heading("Music Player");
                
                // Playlist management buttons - fixed height
                ui.horizontal(|ui| {
                    if ui.button("Add Songs").clicked() {
                        self.add_to_playlist();
                    }
                    
                    if let Some(_index) = self.selected_song_index {
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
                
                // Calculate available space for playlist
                // This is the key part - allocate remaining space between fixed elements
                let available_height = ui.available_height();
                // Reserve space for playback controls and now playing label at bottom
                let bottom_section_height = 70.0;
                let playlist_height = available_height - bottom_section_height;
                
                // Playlist section - takes up remaining space with scroll
                ui.allocate_ui(egui::vec2(ui.available_width(), playlist_height), |ui| {
                    ui.heading("Playlist");
                    
                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .max_height(playlist_height - 30.0) // Account for playlist header
                        .show(ui, |ui| {
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
                                    self.started_playing = true;
                                    self.current_file = Some(path.clone());
                                }
                            }
                        });
                });
                
                ui.separator();
                
                // Bottom controls section - fixed height, always visible
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    // Now playing display
                    if let Some(path) = &self.current_file {
                        ui.label(format!("Now playing: {}", path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("Unknown")));
                    }
                    
                    // Progress bar and time display
                    ui.horizontal(|ui| {
                        // Current position display
                        ui.label(Self::format_duration(self.song_position));
                        
                        // Progress slider
                        let progress_ratio = if let Some(duration) = self.song_duration {
                            if duration.as_secs() > 0 {
                                self.song_position.as_secs_f32() / duration.as_secs_f32()
                            } else {
                                0.0
                            }
                        } else {
                            0.0
                        };
                        
                        let mut seek_pos = if self.seeking {
                            self.seek_position
                        } else {
                            progress_ratio
                        };
                        
                        let slider_response = ui.add(
                            egui::Slider::new(&mut seek_pos, 0.0..=1.0)
                                .show_value(false)
                                .trailing_fill(true)
                        );
                        
                        // Handle seeking
                        if slider_response.drag_started() {
                            self.seeking = true;
                            self.seek_position = seek_pos;
                        } else if slider_response.drag_stopped() {
                            self.seeking = false;
                            self.seek_to_position(seek_pos);
                        } else if slider_response.dragged() {
                            self.seek_position = seek_pos;
                        }
                        
                        // Total duration display
                        if let Some(duration) = self.song_duration {
                            ui.label(Self::format_duration(duration));
                        } else {
                            ui.label("--:--");
                        }
                    });
                    
                    // Playback controls
                    ui.horizontal(|ui| {
                        if self.is_playing {
                            if ui.button("⏸ Pause").clicked() {
                                if let Ok(player) = self.player.lock() {
                                    player.pause();
                                    self.is_playing = false;
                                }
                            }
                        } else if self.current_playlist_index.is_some() && ui.button("▶ Play").clicked() {
                            if let Ok(player) = self.player.lock() {
                                player.resume();
                                self.is_playing = true;
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
                        
                        // Add volume slider
                        ui.add_space(20.0);
                        ui.label("Volume:");
                        let mut volume = self.volume;
                        if ui.add(egui::Slider::new(&mut volume, 0.0..=1.0).show_value(false)).changed() {
                            self.set_volume(volume);
                        }
                        
                        // Show volume percentage
                        ui.label(format!("{}%", (volume * 100.0).round() as i32));
                    });
                });
            });
        });
    }
}

pub fn run(paths: Vec<PathBuf>) -> Result<()> {
    let options = NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size(egui::vec2(500.0, 600.0)),
        ..Default::default()
    };
    
    if eframe::run_native(
        "Music Player",
        options,
        Box::new(|cc| Ok(Box::new(MusicPlayerApp::new(cc, paths)))),
    ).is_err() {
        return Err(anyhow::anyhow!("Failed to run eframe"));
    }

    Ok(())
} 