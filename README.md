# Music Player

A cross-platform music player with both GUI and CLI interfaces. Built with Rust.

## Features

- Cross-platform GUI using eframe (egui)
- Audio playback with rodio
- Playlist management (add, remove, reorder songs)
- Sequential or shuffle playback of songs
- Audio file filtering by extension
- Support for multiple audio formats (MP3, WAV, OGG, FLAC, AAC, etc.)

## Installation

### Build from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/musicplayer.git
cd musicplayer

# Build in release mode
cargo build --release

# The binary will be in target/release/musicplayer
```

### macOS Deployment

For macOS users, a deployment script is included to create a proper `.app` bundle:

```bash
# Make the script executable if needed
chmod +x deploy_macos.sh

# Run the deployment script
./deploy_macos.sh
```

This will:
1. Build the app in release mode
2. Create a proper macOS `.app` bundle with file associations
3. Create a `.zip` file or `.dmg` installer (if `create-dmg` is installed)

To install:
- Copy `MusicPlayer.app` to your Applications folder
- You can then launch the app from Spotlight or Launchpad

To create a DMG installer (optional):
```bash
brew install create-dmg
./deploy_macos.sh
```

### Setting Up File Associations

#### Linux

1. Copy the `musicplayer.desktop` file to your applications directory:

```bash
sudo cp musicplayer.desktop /usr/share/applications/
```

2. Update the desktop database:

```bash
sudo update-desktop-database
```

#### Windows

1. Double-click the `musicplayer_register.reg` file to register file associations.
2. Accept the security prompt.

## Usage

### GUI Mode

```bash
# Launch the GUI with no files
musicplayer

# Launch the GUI with specific files
musicplayer path/to/your/music.mp3 another/song.flac

# Launch the GUI with glob patterns
musicplayer "*.mp3" "playlist/*.wav"
```

### Dropping Files

You can also drag and drop audio files onto the application window to add them to the playlist.

## Supported File Formats

- MP3 (.mp3)
- WAV (.wav)
- OGG Vorbis (.ogg)
- FLAC (.flac)
- AAC (.aac)
- M4A (.m4a)
- Opus (.opus)
- WMA (.wma) 