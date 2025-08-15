# MUSIX

A minimalist terminal-based MP3 music player built with Rust.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Terminal](https://img.shields.io/badge/Terminal-UI-green?style=for-the-badge)
![Music](https://img.shields.io/badge/MP3-Player-orange?style=for-the-badge)

[![asciicast](https://asciinema.org/a/730123.svg)](https://asciinema.org/a/730123)

## Features

- **Beautiful TUI**: Clean terminal interface with cyberpunk green theme
- **High-Quality Playback**: MP3 audio support with crystal-clear sound
- **Visual Progress**: Real-time progress bar with time display
- **Smart Controls**: Intuitive keyboard controls with popup help
- **Smooth Seeking**: Instant seek without playback interruption
- **Playback Modes**: Normal sequential and random shuffle
- **Keyboard-Driven**: Lightning-fast keyboard-only interface
- **Fuzzy Search**: Real-time search with `/` key - find songs instantly
- **Vim-Style Navigation**: Full vim keybinding support (hjkl, gg/G, n/N, q)

## Quick Start

### Prerequisites

- **Rust 1.70+** - [Install Rust](https://rustup.rs/)
- **Audio Libraries** (Linux): `libasound2-dev pkg-config`

### Installation

```bash
# Clone the repository
git clone https://github.com/harukiinharu/musix.git
cd musix

# Build and run
cargo run

# Or build optimized release version
cargo build --release
./target/release/musix
```

### Quick Usage
1. **Start the player**: `cargo run`
2. **Navigate**: Use `j/k` or arrow keys to browse songs
3. **Search**: Press `/` and type to find songs instantly
4. **Play**: Press `Enter` or `Space` to play selected song
5. **Jump**: Use `g` (first song) or `G` (last song)
6. **Help**: Press `x` to see all controls
7. **Quit**: Press `q` or `Esc` to exit

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

> **Tip**: Press **x** anytime to view the interactive controls popup!

### Essential Keys

| Key | Action |
|-----|--------|
| **`Space/↵`** | **Smart Play** - Play selected song or pause current |
| **`/`** | **Search Mode** - Enter fuzzy search |
| **`x`** | **Show/Hide help popup** |
| **`q/Esc`** | **Exit** |

### Navigation & Playback

| Key | Action |
|-----|--------|
| `↑/↓` or `j/k` | Navigate songs (vim-style) |
| `Space/↵` | Play/pause (same functionality) |
| `←/→` or `h/l` | Play previous/next song |
| `g` / `G` | Jump to first/last song |
| `,` / `.` | Seek backward/forward 5 seconds |
| `<` / `>` | Same as above |
| `r` | Toggle Random mode |

### Search Mode

| Key | Action |
|-----|--------|
| **`/`** | Enter search mode |
| `n` / `N` | Navigate to next/previous search result |
| `↑/↓` or `j/k` | Navigate through filtered results |
| `Enter` | Play selected song and exit search |
| `Esc` | Exit search mode |
| `Backspace` | Delete characters from search query |
| `Any text` | Type to search (fuzzy matching) |

## Interface

MUSIX features a clean, 4-panel interface that maximizes space for your music:

```
┌─────────────────────────────────┐
│             MUSIX               │  ← Title Bar
├─────────────────────────────────┤
│ Songs - Search: rock            │  ← Song List (Search Mode)
│ → ♪ 1. Rock Song                │    or "Songs" (Normal Mode)
│     5. Another Rock Song        │    (Scrollable, Filtered)
│     12. Rock Ballad             │
│     More filtered results...    │
├─────────────────────────────────┤
│ ████████████████░░░░ 02:30/04:15│  ← Progress Bar
├─────────────────────────────────┤
│ Search Mode | Songs: 15/120 |.. │  ← Status & Search Info
└─────────────────────────────────┘
```

### Interactive Controls Popup (Press **x**)

```
┌─────────────────────────────────┐
│            CONTROLS             │
│                                 │
│ ↑/↓ or j/k - Navigate songs     │
│ Space/↵    - Play/Pause         │
│ ←/→ or h/l - Play prev/next song│
│ g/G       - Jump to first/last │
│ /          - Enter search mode  │
│ n/N        - Next/prev search   │
│ ,/.        - Seek ±5 seconds    │
│ r          - Toggle random mode │
│ q/Esc      - Exit application   │
│ x          - Close this popup   │
└─────────────────────────────────┘
```

## Smart Features

### Fuzzy Search
- **Instant Search**: Press `/` to enter search mode
- **Real-time Filtering**: Results update as you type
- **Fuzzy Matching**: Finds songs even with partial or misspelled text
- **Smart Scoring**: Prioritizes exact matches → substring matches → fuzzy matches
- **Search Navigation**: Use `n/N` to quickly jump between results
- **Quick Play**: Press Enter on any result to play immediately

**Example**: Searching "btl" will match "Battle Song", "Beautiful", "Subtitle"

### Visual Indicators
- **`→`** Currently selected song in the list
- **`♪`** Currently playing song indicator  
- **Progress Bar** Real-time playback progress with time
- **Search Title** Shows current search query in song list header
- **Result Count** Displays filtered results count (e.g., "15/120 songs")

### Playback Modes
- **Normal Mode**: Sequential playback through your playlist
- **Random Mode**: Intelligent shuffle (excludes current song)

### Smart Space/Enter Key
- **Initial state**: Plays the first selected song
- **Different song selected**: Plays the selected song immediately
- **Same song selected**: Toggles play/pause for current song

### Vim-Style Navigation
- **Movement**: `hjkl` for navigation (h=left, j=down, k=up, l=right)
- **Jumping**: `g` jumps to first song, `G` jumps to last song
- **Search Navigation**: `n/N` for next/previous search results
- **Quit**: `q` as alternative to Escape

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
