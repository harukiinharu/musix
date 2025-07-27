# MUSIX

A minimalist terminal-based MP3 music player built with Rust.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Terminal](https://img.shields.io/badge/Terminal-UI-green?style=for-the-badge)
![Music](https://img.shields.io/badge/MP3-Player-orange?style=for-the-badge)

[![asciicast](https://asciinema.org/a/45pMbZkgYuKoOqfyqeRpoR6BS.svg)](https://asciinema.org/a/45pMbZkgYuKoOqfyqeRpoR6BS)

## Features

- **Beautiful TUI**: Clean terminal interface with cyberpunk green theme
- **High-Quality Playback**: MP3 audio support with crystal-clear sound
- **Visual Progress**: Real-time progress bar with time display
- **Smart Controls**: Intuitive keyboard controls with popup help (Press **X**)
- **Smooth Seeking**: Instant seek without playback interruption
- **Playback Modes**: Normal sequential and random shuffle
- **Keyboard-Driven**: Lightning-fast keyboard-only interface

## Quick Start

### Prerequisites

- **Rust 1.70+** - [Install Rust](https://rustup.rs/)
- **Audio Libraries** (Linux): `libasound2-dev pkg-config`

### Installation

```bash
# Clone the repository
git clone git@github.com:coolcode/musix.git
cd musix

# Build and run
cargo run

# Or build optimized release version
cargo build --release
./target/release/musix
```

### Setup Music Files

MUSIX automatically searches for MP3 files in these directories:

1. **`~/Music`** - Your system's Music directory
2. **`./data`** - Local data folder

```bash
# Option 1: Use local data folder
mkdir -p ./data
cp /path/to/your/music/*.mp3 ./data/

# Option 2: Use system Music directory
cp /path/to/your/music/*.mp3 ~/Music/

# Option 3: Create symbolic link
ln -s /path/to/your/music ./data
```

## Controls

> **Tip**: Press **X** anytime to view the interactive controls popup!

### Essential Keys

| Key | Action |
|-----|--------|
| **`Space/↵`** | **Smart Play** - Play selected song or pause current |
| **`X`** | **Show/Hide help popup** |
| **`Esc`** | **Exit** |

### Navigation & Playback

| Key | Action |
|-----|--------|
| `↑` / `↓` | Navigate songs |
| `Space/↵` | Play/pause (same functionality) |
| `←` / `→` | Play previous/next song |
| `,` / `.` | Seek backward/forward 5 seconds |
| `<` / `>` | Same as above |
| `R` | Toggle Random mode |

## Interface

MUSIX features a clean, 4-panel interface that maximizes space for your music:

```
┌─────────────────────────────────┐
│             MUSIX               │  ← Title Bar
├─────────────────────────────────┤
│ → ♪ 1. Current Song             │  ← Song List
│     2. Another Song             │    (Scrollable)
│     3. Third Song               │
│     4. More songs...            │
├─────────────────────────────────┤
│ ████████████████░░░░ 02:30/04:15│  ← Progress Bar
├─────────────────────────────────┤
│ Mode: ___ | Songs: 20 | X: Help │  ← Status & Help
└─────────────────────────────────┘
```

### Interactive Controls Popup (Press **X**)

```
┌─────────────────────────────────┐
│            CONTROLS             │
│                                 │
│ ↑/↓ - Navigate songs            │
│ Space/↵ - Play Pause            │
│ ←/→ - Play prev/next song       │
│ ,/. - Seek ±5 seconds           │
│ R - Toggle random mode          │
│ X - Close this popup            │
│ Esc - Exit application          │
└─────────────────────────────────┘
```

## Smart Features

### Visual Indicators
- **`→`** Currently selected song in the list
- **`♪`** Currently playing song indicator  
- **Progress Bar** Real-time playback progress with time

### Playback Modes
- **Normal Mode**: Sequential playback through your playlist
- **Random Mode**: Intelligent shuffle (excludes current song)

### Smart Space/Enter Key
- **Initial state**: Plays the first selected song
- **Different song selected**: Plays the selected song immediately
- **Same song selected**: Toggles play/pause for current song

## Technical Details

### Architecture
- **Player Engine**: State management with smart playback control
- **Terminal UI**: Ratatui-powered responsive interface  
- **Audio Engine**: Rodio-based high-quality MP3 processing
- **Performance**: Efficient seeking without playback interruption

### Core Dependencies
- **`rodio`** - Professional audio playback and MP3 decoding
- **`ratatui`** - Modern terminal user interface framework
- **`crossterm`** - Cross-platform terminal control
- **`rand`** - Cryptographically secure random shuffle

## Development

### Project Structure

```
musix/
├── src/
│   └── main.rs          # Complete application (~700 lines)
├── data/                # MP3 files (optional)
├── .github/workflows/   # CI/CD automation
├── Cargo.toml          # Dependencies and metadata
├── rustfmt.toml        # Code formatting rules
└── README.md           # Documentation
```

### Building & Testing

```bash
# Development build
cargo build

# Optimized release build
cargo build --release

# Run all tests
cargo test

# Code quality checks
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
```

## Troubleshooting

### No Music Files Found

**Issue**: `No MP3 files found in any accessible directory`

**Solutions**:
```bash
# Option 1: Copy files to data folder
mkdir -p ./data
cp /path/to/your/music/*.mp3 ./data/

# Option 2: Create symbolic link
ln -s /path/to/your/music ./data

# Option 3: Check permissions
ls -la ~/Music
```

### macOS Music Access

**Issue**: Cannot access ~/Music directory on macOS

**Solution**: Enable Full Disk Access for your terminal:

1. **System Settings** → **Privacy & Security** → **Full Disk Access**
2. Click lock icon to unlock settings
3. Click **+** and add your terminal app (Terminal/iTerm2)
4. Enable the checkbox
5. **Restart your terminal**

### Linux Audio Issues

**Issue**: No audio output or initialization errors

**Solutions**:
```bash
# Install required audio libraries
sudo apt-get update
sudo apt-get install libasound2-dev pkg-config

# For other distributions
sudo pacman -S alsa-lib pkg-config  # Arch
sudo dnf install alsa-lib-devel pkgconf  # Fedora
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## Acknowledgments

- **Rodio** team for excellent Rust audio library
- **Ratatui** team for powerful TUI framework
- **Rust** community for amazing ecosystem

---

**Built with Claud Code**
