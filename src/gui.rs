use anyhow::Result;
use eframe::{ egui, egui::ViewportBuilder, NativeOptions };
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use crate::player::MusicPlayer;

struct MusicPlayerApp {
    player: Arc<Mutex<MusicPlayer>>,
    current_file: Option<PathBuf>,
    started_playing: bool,
}

impl MusicPlayerApp {
    fn new(cc: &eframe::CreationContext<'_>, path: Option<String>) -> Self {
        let mut file: Option<PathBuf> = None;
        let mut started_playing: bool = false;
        if let Some(path) = path {
            let path = PathBuf::from(path);
            if path.is_file() {
                file = Some(path);
                started_playing = true;
            }
        }

        Self {
            player: Arc::new(Mutex::new(MusicPlayer::new().unwrap())),
            current_file: file,
            started_playing: started_playing,
        }
    }
}

impl eframe::App for MusicPlayerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.started_playing {
            self.started_playing = false;
            if let Some(path) = &self.current_file {
                if let Ok(mut player) = self.player.lock() {
                    let _ = player.play_file(&path);
                }
            }
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Music Player");
            
            if ui.button("Open File").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Audio Files", &["mp3", "wav", "ogg", "flac"])
                    .pick_file() 
                {
                    self.current_file = Some(path.clone());
                    if let Ok(mut player) = self.player.lock() {
                        let _ = player.play_file(&path);
                    }
                }
            }

            ui.separator();

            if let Some(path) = &self.current_file {
                ui.label(format!("Current file: {}", path.display()));
                
                if let Ok(player) = self.player.lock() {
                    if player.is_playing() {
                        if ui.button("Pause").clicked() {
                            player.pause();
                        }
                    } else {
                        if ui.button("Resume").clicked() {
                            player.resume();
                        }
                    }
                    
                    if ui.button("Stop").clicked() {
                        player.stop();
                    }
                }
            }
        });
    }
}

pub fn run(path: Option<String>) -> Result<()> {
    let options = NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size(egui::vec2(400.0, 300.0)),
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