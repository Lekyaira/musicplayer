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

#### macOS

1. Build the application bundle:

```bash
# Create app bundle structure
mkdir -p MusicPlayer.app/Contents/MacOS
mkdir -p MusicPlayer.app/Contents/Resources

# Copy executable to the bundle
cp target/release/musicplayer MusicPlayer.app/Contents/MacOS/

# Copy Info.plist
cp Info.plist.template MusicPlayer.app/Contents/Info.plist

# Create an icon file (optional)
# cp AppIcon.icns MusicPlayer.app/Contents/Resources/
```

2. To register the app with the system:
   - Move the app to the Applications folder
   - Right-click on an audio file
   - Select "Open With" → "Other..."
   - Navigate to your app and check "Always Open With"

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