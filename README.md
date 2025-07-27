# Musix

A minimalist terminal-based MP3 music player built with Rust.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Terminal](https://img.shields.io/badge/Terminal-UI-green?style=for-the-badge)
![Music](https://img.shields.io/badge/MP3-Player-orange?style=for-the-badge)

[![asciicast](https://asciinema.org/a/45pMbZkgYuKoOqfyqeRpoR6BS.svg)](https://asciinema.org/a/45pMbZkgYuKoOqfyqeRpoR6BS)

## Features

- **Beautiful TUI**: Modern terminal interface with cyberpunk green theme
- **MP3 Playback**: Supports MP3 audio files with high-quality playback
- **Progress Bar**: Visual progress tracking with elapsed/total time display
- **Full Controls**: Play, pause, resume, seek, skip, and shuffle
- **Playback Modes**: Normal and Random shuffle modes
- **Fast Navigation**: Keyboard-driven interface for quick song selection

## Quick Start

### Prerequisites

- Rust 1.70+

### Installation

```bash
# Clone the repository
git clone git@github.com:coolcode/musix.git
cd musix

# Build and run
cargo run

# Or build release version
cargo build --release
./target/release/musix
```

### Setup Music Files

MUSIX searches for MP3 files in these directories (in order):

1. `~/Music` (User's music directory)
2. `./data` (Current directory)

```bash
# Copy your MP3 files to the data directory for testing
mkdir -p ./data
cp /path/to/your/music/*.mp3 ./data/

# Or use your system's Music directory
cp /path/to/your/music/*.mp3 ~/Music/
```

## Controls

| Key | Action |
|-----|--------|
| `↑` / `↓` | Navigate song list |
| `Enter` / `Space` / `P` | Play selected song |
| `S` | Pause/Resume playback |
| `←` / `→` | Previous/Next song |
| `<` / `>` | Seek backward/forward 5 seconds |
| `R` | Toggle Random mode |
| `Esc` / `Ctrl+C` | Exit |

## Interface Layout

```
┌─────────────────────────────────┐
│             MUSIX               │  ← Title
├─────────────────────────────────┤
│ → ♪ 1. Current Song             │  ← Song List
│     2. Another Song             │    (with selection)
│     3. Third Song               │
├─────────────────────────────────┤
│ Progress ████████░░░ 02:30/04:15│  ← Progress Bar
├─────────────────────────────────┤
│ ↑/↓: Select | Enter/P: Play    │  ← Controls Help
│ S: Pause/Resume | ←/→: Prev/Next│
│ </>: Seek ±5s | R: Random      │
├─────────────────────────────────┤
│ Mode: NORMAL | Songs: 5 | ⏵... │  ← Status
└─────────────────────────────────┘
```

## Visual Indicators

- **→** : Currently selected song
- **♪** : Currently playing song
- **⏵** : Playing status in status bar
- **⏸** : Paused status in status bar

## Technical Features

### Playback Modes

- **Normal**: Sequential playback through playlist
- **Random**: Randomly select next song (excluding current)

## Architecture

### Core Components

- **Player**: Main playback engine with state management
- **UI**: Ratatui-based terminal interface with real-time updates
- **Audio Engine**: Rodio-based MP3 decoding and playback

### Dependencies

- `rodio` - Audio playback and MP3 decoding
- `ratatui` - Terminal user interface framework
- `crossterm` - Cross-platform terminal manipulation
- `rand` - Random number generation for shuffle mode

## Development

### Project Structure

```
musix/
├── src/
│   └── main.rs          # Complete application (~730 lines)
├── data/                # MP3 files (optional)
├── Cargo.toml          # Dependencies and metadata
└── README.md           # This file
```

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test
```

### Permission Denied

**Problem**: Cannot access music directories  
**Solution**:

- Check directory permissions: `ls -la ~/Music`
- Copy files to `./data` directory instead

** Access Apple Music**

Enabling Full Disk Access for Terminal/iTerm2

Step-by-step instructions

1. Click the Apple logo () in the top-left corner and open System Settings (or System Preferences).
2. Navigate to Privacy & Security in the sidebar.
3. Scroll down and click Full Disk Access.
4. Click the lock icon at the bottom-left to unlock the pane (you will need to authenticate as an administrator).  ￼ ￼
5. Click the "+" button to add an app, then choose Terminal/iTerm2.  ￼
6. Ensure the checkbox next to Terminal/iTerm2 is enabled.
7. Exit settings and restart Terminal/iTerm2 for changes to take effect.  

### No MP3 Files Found

**Problem**: "No MP3 files found in any accessible directory"  
**Solution**:
```bash
# Copy test files
mkdir -p ./data
cp /path/to/music/*.mp3 ./data/

# Or create symbolic link
ln -s /path/to/music ./data
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

**Built with Rust**