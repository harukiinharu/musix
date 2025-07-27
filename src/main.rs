use std::{
    fs, io,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use rand::prelude::*;
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, ListState, Paragraph},
};
use rodio::{decoder::DecoderBuilder, Decoder, OutputStreamBuilder, Sink, Source};

#[derive(Clone)]
struct Song {
    name: String,
    path: PathBuf,
}

const HIGHLIGHT_COLOR: Color = Color::Rgb(0, 255, 150);
const PRIMARY_COLOR: Color = Color::LightGreen;

struct Player {
    songs: Vec<Song>,
    current_index: usize,
    selected_index: usize,
    _stream_handle: Option<Box<dyn std::any::Any>>,
    sink: Option<Arc<Mutex<Sink>>>,
    is_playing: bool,
    loop_mode: bool,
    random_mode: bool,
    list_state: ListState,
    playback_start: Option<Instant>,
    song_duration: Option<Duration>,
    seek_offset: Duration,
}

impl Player {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let songs = load_mp3_files()?;
        if songs.is_empty() {
            return Err("No MP3 files found".into());
        }

        let mut list_state = ListState::default();
        list_state.select(Some(0));

        // Initialize audio system with Rodio 0.21 API
        let (stream_handle, sink) = match OutputStreamBuilder::open_default_stream() {
            Ok(stream_handle) => {
                let sink = Sink::connect_new(&stream_handle.mixer());
                eprintln!("Audio system initialized successfully.");
                (Some(Box::new(stream_handle) as Box<dyn std::any::Any>), Some(Arc::new(Mutex::new(sink))))
            }
            Err(e) => {
                eprintln!("Warning: Could not initialize audio output: {}", e);
                eprintln!("The application will continue but audio playback may not work.");
                (None, None)
            }
        };

        Ok(Player {
            songs,
            current_index: 0,
            selected_index: 0,
            _stream_handle: stream_handle,
            sink,
            is_playing: false,
            loop_mode: true,
            random_mode: false,
            list_state,
            playback_start: None,
            song_duration: None,
            seek_offset: Duration::from_secs(0),
        })
    }

    fn play_song(&mut self, index: usize) -> Result<(), Box<dyn std::error::Error>> {
        if index >= self.songs.len() {
            return Ok(());
        }

        self.current_index = index;
        self.seek_offset = Duration::from_secs(0);
        if let Some(ref sink) = self.sink {
            let song = &self.songs[index];
            match std::fs::File::open(&song.path) {
                Ok(file) => {
                    match Decoder::try_from(file) {
                        Ok(source) => {
                            // Try to get duration from the source
                            let total_duration = source.total_duration();

                            let sink = sink.lock().unwrap();
                            sink.stop();

                            // If we have a seek offset, we need to skip ahead
                            if self.seek_offset > Duration::from_secs(0) {
                                let skipped_source = source.skip_duration(self.seek_offset);
                                sink.append(skipped_source);
                            } else {
                                sink.append(source);
                            }

                            sink.play();
                            self.is_playing = true;
                            self.playback_start = Some(Instant::now());
                            self.song_duration = total_duration;
                        }
                        Err(e) => {
                            eprintln!("Warning: Could not decode audio file '{}': {}", song.name, e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Warning: Could not open audio file '{}': {}", song.name, e);
                }
            }
        } else {
            eprintln!("Warning: No audio sink available. Cannot play '{}'", self.songs[index].name);
        }

        Ok(())
    }

    fn next_song(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.songs.is_empty() {
            return Ok(());
        }

        let next_index = if self.random_mode {
            let mut rng = rand::rng();
            let mut indices: Vec<usize> = (0..self.songs.len()).collect();
            indices.remove(self.current_index);
            if indices.is_empty() {
                self.current_index
            } else {
                *indices.choose(&mut rng).unwrap()
            }
        } else if self.current_index + 1 >= self.songs.len() {
            if self.loop_mode { 0 } else { self.current_index }
        } else {
            self.current_index + 1
        };

        self.play_song(next_index)
    }

    fn previous_song(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.songs.is_empty() {
            return Ok(());
        }

        let prev_index = if self.random_mode {
            let mut rng = rand::rng();
            let mut indices: Vec<usize> = (0..self.songs.len()).collect();
            indices.remove(self.current_index);
            if indices.is_empty() {
                self.current_index
            } else {
                *indices.choose_mut(&mut rng).unwrap()
            }
        } else if self.current_index == 0 {
            if self.loop_mode { self.songs.len() - 1 } else { 0 }
        } else {
            self.current_index - 1
        };

        self.play_song(prev_index)
    }

    fn move_selection(&mut self, direction: i32) {
        if self.songs.is_empty() {
            return;
        }

        let len = self.songs.len();
        if direction > 0 {
            self.selected_index = (self.selected_index + 1) % len;
        } else if direction < 0 {
            self.selected_index = if self.selected_index == 0 { len - 1 } else { self.selected_index - 1 };
        }
        self.list_state.select(Some(self.selected_index));
    }

    fn get_playback_progress(&self) -> (Duration, Option<Duration>) {
        if let Some(start_time) = self.playback_start {
            let elapsed = start_time.elapsed() + self.seek_offset;
            (elapsed, self.song_duration)
        } else {
            (self.seek_offset, self.song_duration)
        }
    }

    fn format_duration(duration: Duration) -> String {
        let total_seconds = duration.as_secs();
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        format!("{:02}:{:02}", minutes, seconds)
    }

    fn pause_playback(&mut self) {
        if self.is_playing {
            // Store current progress before pausing
            if let Some(start_time) = self.playback_start {
                self.seek_offset += start_time.elapsed();
            }

            if let Some(ref sink) = self.sink {
                let sink = sink.lock().unwrap();
                sink.pause();
            }
            self.is_playing = false;
            self.playback_start = None;
        }
    }

    fn resume_playback(&mut self) {
        if !self.is_playing && !self.songs.is_empty() {
            if let Some(ref sink) = self.sink {
                let sink = sink.lock().unwrap();
                sink.play();
                self.is_playing = true;
                self.playback_start = Some(Instant::now());
            }
        }
    }

    fn seek(&mut self, offset_seconds: i32) {
        if !self.songs.is_empty() && self.is_playing {
            // Get current actual position (including elapsed time since playback start)
            let current_position = if let Some(start_time) = self.playback_start {
                self.seek_offset + start_time.elapsed()
            } else {
                self.seek_offset
            };

            let seek_duration = Duration::from_secs(offset_seconds.abs() as u64);
            let new_position = if offset_seconds < 0 {
                // Seek backward
                if current_position > seek_duration {
                    current_position - seek_duration
                } else {
                    Duration::from_secs(0)
                }
            } else {
                // Seek forward
                let new_pos = current_position + seek_duration;
                // Don't seek past song duration if we know it
                if let Some(duration) = self.song_duration {
                    if new_pos >= duration {
                        duration.saturating_sub(Duration::from_secs(1))
                    } else {
                        new_pos
                    }
                } else {
                    new_pos
                }
            };

            // Update seek offset and reset playback start time to simulate seeking
            self.seek_offset = new_position;
            self.playback_start = Some(Instant::now());
        }
    }
}

fn load_mp3_files() -> Result<Vec<Song>, Box<dyn std::error::Error>> {
    let mut songs = Vec::new();

    // Try multiple directories in order of preference
    let potential_dirs = vec![
        {
            // User's Music directory
            let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            PathBuf::from(format!("{}/Music", home_dir))
        },
        PathBuf::from("./data"),
    ];

    for data_dir in potential_dirs {
        if data_dir.exists() {
            match visit_dir(&data_dir, &mut songs) {
                Ok(_) => {
                    eprintln!("Loaded {} MP3 files from: {:?}", songs.len(), data_dir);
                    // break;
                }
                Err(e) => {
                    eprintln!("Warning: Could not access directory {:?}: {}", data_dir, e);
                    continue;
                }
            }
        }
    }

    songs.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(songs)
}

fn visit_dir(dir: &PathBuf, songs: &mut Vec<Song>) -> Result<(), Box<dyn std::error::Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                visit_dir(&path, songs)?;
            } else if let Some(extension) = path.extension() {
                if extension.to_str().unwrap_or("").to_lowercase() == "mp3" {
                    let name = path.file_stem().and_then(|s| s.to_str()).unwrap_or("Unknown").to_string();

                    songs.push(Song { name, path: path.clone() });
                }
            }
        }
    }
    Ok(())
}

fn ui(f: &mut Frame, player: &Player) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(8),    // Song list
            Constraint::Length(3), // Progress bar
            Constraint::Length(4), // Controls
            Constraint::Length(3), // Status
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("MUSIX")
        .style(Style::default().fg(PRIMARY_COLOR).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(PRIMARY_COLOR)));
    f.render_widget(title, chunks[0]);

    // Song list
    let items: Vec<ListItem> = player
        .songs
        .iter()
        .enumerate()
        .map(|(i, song)| {
            let playing_indicator = if i == player.current_index && player.is_playing { "♪ " } else { "  " };

            let content = format!("{}{}. {}", playing_indicator, i + 1, song.name);

            let style = if i == player.selected_index {
                Style::default().fg(PRIMARY_COLOR)
            } else if i == player.current_index && player.is_playing {
                Style::default().fg(HIGHLIGHT_COLOR).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            ListItem::new(content).style(style)
        })
        .collect();

    let songs_list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Songs").border_style(Style::default().fg(PRIMARY_COLOR)))
        .highlight_style(Style::default().fg(PRIMARY_COLOR).add_modifier(Modifier::BOLD))
        .scroll_padding(1);

    f.render_stateful_widget(songs_list, chunks[1], &mut player.list_state.clone());

    // Progress bar
    let (elapsed, total) = player.get_playback_progress();
    let progress_ratio = if let Some(duration) = total {
        if duration.as_secs() > 0 {
            (elapsed.as_secs() as f64 / duration.as_secs() as f64).min(1.0)
        } else {
            0.0
        }
    } else {
        0.0
    };

    let progress_label_text = if let Some(duration) = total {
        format!("{}/{}", Player::format_duration(elapsed), Player::format_duration(duration))
    } else {
        format!("{}", Player::format_duration(elapsed))
    };

    let progress_label = Span::styled(progress_label_text, Style::default().fg(Color::White));

    let progress_bar = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Progress").border_style(Style::default().fg(PRIMARY_COLOR)))
        .gauge_style(Style::default().fg(PRIMARY_COLOR))
        .ratio(progress_ratio)
        .label(progress_label);
    f.render_widget(progress_bar, chunks[2]);

    // Controls
    let controls = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("↑/↓", Style::default().fg(PRIMARY_COLOR).add_modifier(Modifier::BOLD)),
            Span::raw(": Select | "),
            Span::styled("Enter/Space/P", Style::default().fg(PRIMARY_COLOR).add_modifier(Modifier::BOLD)),
            Span::raw(": Play | "),
            Span::styled("S", Style::default().fg(PRIMARY_COLOR).add_modifier(Modifier::BOLD)),
            Span::raw(": Pause/Resume | "),
            Span::styled("←/→", Style::default().fg(PRIMARY_COLOR).add_modifier(Modifier::BOLD)),
            Span::raw(": Prev/Next"),
        ]),
        Line::from(vec![
            Span::styled("</>", Style::default().fg(PRIMARY_COLOR).add_modifier(Modifier::BOLD)),
            Span::raw(": Seek ±5s | "),
            Span::styled("R", Style::default().fg(PRIMARY_COLOR).add_modifier(Modifier::BOLD)),
            Span::raw(": Random | "),
            Span::styled("Esc", Style::default().fg(PRIMARY_COLOR).add_modifier(Modifier::BOLD)),
            Span::raw(": Exit"),
        ]),
    ])
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL).title("Controls").border_style(Style::default().fg(PRIMARY_COLOR)));
    f.render_widget(controls, chunks[3]);

    // Status
    let mode_text = if player.random_mode { "RANDOM" } else { "NORMAL" };

    let song_text = if player.songs.is_empty() {
        String::new()
    } else {
        if player.is_playing {
            format!("⏵ {}", player.songs[player.current_index].name)
        } else {
            format!("⏸ {}", player.songs[player.current_index].name)
        }
    };

    let status_text = format!("  Mode: {} | Songs: {} | {}  ", mode_text, player.songs.len(), song_text);

    let status = Paragraph::new(status_text)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .block(Block::default().borders(Borders::ALL).title("Status").border_style(Style::default().fg(PRIMARY_COLOR)));
    f.render_widget(status, chunks[4]);
}

fn run_player() -> Result<(), Box<dyn std::error::Error>> {
    let mut player = match Player::new() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Player initialization failed: {}", e);
            eprintln!("Error details: {:?}", e);
            std::process::exit(1);
        }
    };

    if player.songs.is_empty() {
        println!("No MP3 files found in any accessible directory.");
        println!("MUSIX searched for MP3 files in:");
        println!("  - ~/Music (user's music directory)");
        println!("  - ./data (current directory)");
        println!();
        println!("To test MUSIX, you can:");
        println!("Copy MP3 files to ./data directory");
        return Ok(());
    }

    match enable_raw_mode() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to enable raw mode: {}", e);
            return Err(e.into());
        }
    }

    let mut stdout = io::stdout();
    match execute!(stdout, EnterAlternateScreen) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to enter alternate screen: {}", e);
            return Err(e.into());
        }
    }

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = match Terminal::new(backend) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to create terminal: {}", e);
            return Err(e.into());
        }
    };

    let result = main_loop(&mut terminal, &mut player);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn main_loop(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, player: &mut Player) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|f| ui(f, player))?;

        if let Ok(true) = event::poll(Duration::from_millis(100)) {
            if let Ok(event) = event::read() {
                if let Event::Key(key) = event {
                    match key {
                        KeyEvent {
                            code: KeyCode::Esc,
                            modifiers: KeyModifiers::NONE,
                            ..
                        }
                        | KeyEvent {
                            code: KeyCode::Char('c'),
                            modifiers: KeyModifiers::CONTROL,
                            ..
                        } => break,

                        KeyEvent {
                            code: KeyCode::Up,
                            modifiers: KeyModifiers::NONE,
                            ..
                        } => {
                            player.move_selection(-1);
                        }

                        KeyEvent {
                            code: KeyCode::Down,
                            modifiers: KeyModifiers::NONE,
                            ..
                        } => {
                            player.move_selection(1);
                        }

                        KeyEvent {
                            code: KeyCode::Enter | KeyCode::Char(' ') | KeyCode::Char('p'),
                            modifiers: KeyModifiers::NONE,
                            ..
                        } => {
                            player.play_song(player.selected_index)?;
                        }

                        KeyEvent {
                            code: KeyCode::Left,
                            modifiers: KeyModifiers::NONE,
                            ..
                        } => {
                            player.previous_song()?;
                        }

                        KeyEvent {
                            code: KeyCode::Right,
                            modifiers: KeyModifiers::NONE,
                            ..
                        } => {
                            player.next_song()?;
                        }

                        KeyEvent {
                            code: KeyCode::Char('r'),
                            modifiers: KeyModifiers::NONE,
                            ..
                        } => {
                            player.random_mode = !player.random_mode;
                        }

                        KeyEvent {
                            code: KeyCode::Char('s'),
                            modifiers: KeyModifiers::NONE,
                            ..
                        } => {
                            if player.is_playing {
                                player.pause_playback();
                            } else {
                                player.resume_playback();
                            }
                        }

                        KeyEvent {
                            code: KeyCode::Char('<') | KeyCode::Char(','),
                            modifiers: KeyModifiers::NONE,
                            ..
                        } => {
                            player.seek(-5); // Seek backward 5 seconds
                        }

                        KeyEvent {
                            code: KeyCode::Char('>') | KeyCode::Char('.'),
                            modifiers: KeyModifiers::NONE,
                            ..
                        } => {
                            player.seek(5); // Seek forward 5 seconds
                        }

                        _ => {}
                    }
                }
            }
        }

        // Check if current song finished and auto-play next
        if player.is_playing {
            if let Some(ref sink) = player.sink {
                let sink = sink.lock().unwrap();
                if sink.empty() {
                    drop(sink);
                    player.is_playing = false;
                    player.playback_start = None;
                    player.seek_offset = Duration::from_secs(0);
                    player.next_song()?;
                }
            }
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run_player() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
