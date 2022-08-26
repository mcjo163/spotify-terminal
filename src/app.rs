use std::time::{Duration, Instant};
use std::io;
use termion::event::Key;
use termion::color;

use crate::{
    Terminal, 
    Position,
    Sptfy,
};

// key constants needed by other modules
pub const QUIT_KEY: Key = Key::Ctrl('q');

const SIDE_PANE_COLOR: color::Rgb = color::Rgb(18, 18, 20);
const CENTER_PANE_COLOR: color::Rgb = color::Rgb(38, 38, 40);
const STATUS_BAR_COLOR: color::Rgb = color::Rgb(48, 48, 50);
const SPOTIFY_GREEN: color::Rgb = color::Rgb(30, 215, 96);
const PLAYED_COLOR: color::Rgb = color::Rgb(210, 210, 212);
const UNPLAYED_COLOR: color::Rgb = color::Rgb(130, 130, 132);

struct AppState {
    playing: bool,
    song_length: Duration,
    song_progress: Duration,
}

impl AppState {
    fn default() -> Self {
        Self {
            playing: false,
            song_length: Duration::from_secs(30),
            song_progress: Duration::from_secs(0),
        }
    }

    fn toggle_playing(&mut self) {
        self.playing = !self.playing;
    }

    fn get_progress_val(&self) -> f32 {
        self.song_progress.as_secs_f32() / self.song_length.as_secs_f32()
    }
}

pub struct App {
    quit_requested: bool,
    terminal: Terminal,
    _sptfy: Sptfy,
    state: AppState,
}

impl App {

    pub fn default() -> Self {
        App {
            quit_requested: false,
            terminal: Terminal::default().expect("failed to initialize terminal"),
            _sptfy: Sptfy::default(),
            state: AppState::default(),
        }
    }

    pub fn run(&mut self) {
        loop {
            let now = Instant::now();
            if let Err(e) = self.refresh_screen() {
                die(&e);
            }
            if self.quit_requested {
                break;
            }
            if let Err(e) = self.handle_keys() {
                die(&e);
            }

            // target 10 FPS (redraw every 100ms)
            std::thread::sleep(Duration::from_millis(100 - (now.elapsed().as_millis()) as u64));
            if self.state.playing {
                self.state.song_progress += Duration::from_millis(100);
                if self.state.song_progress >= self.state.song_length {
                    self.state.song_progress = Duration::from_secs(0);
                }
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), io::Error> {

        // prepare frame
        Terminal::cursor_position(&Position::default());
        Terminal::cursor_hide();

        if self.quit_requested {
            // clean up display before exiting
            Terminal::clear_screen();
            Terminal::cursor_show();
        } else {
            // render next frame
            self.render();
        }

        Terminal::reset_fg_color();
        Terminal::reset_bg_color();
        Terminal::flush()
    }

    fn render(&self) {
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;

        // calculate dimensions
        let side_pane_width = 3 * width / 10;
        let center_pane_width = width - 2 * side_pane_width;
        let pane_height = height - 3;

        // top green bar
        Terminal::set_bg_color(SPOTIFY_GREEN);
        Terminal::set_fg_color(SIDE_PANE_COLOR);
        let title_msg = "spotify-terminal";
        let corner_msg = "ctrl-q to quit";
        let padding = " ".to_string().repeat(width - title_msg.len() - corner_msg.len());
        println!("{title_msg}{padding}{corner_msg}\r");
        
        let side_pane_row = " ".to_string().repeat(side_pane_width);
        let center_pane_row = " ".to_string().repeat(center_pane_width);

        for _ in 1..pane_height {
            Terminal::set_bg_color(SIDE_PANE_COLOR);
            print!("{side_pane_row}");
            Terminal::set_bg_color(CENTER_PANE_COLOR);
            print!("{center_pane_row}");
            Terminal::set_bg_color(SIDE_PANE_COLOR);
            println!("{side_pane_row}\r");
        }

        Terminal::set_bg_color(STATUS_BAR_COLOR);
        let blank_row = " ".to_string().repeat(width);
        println!("{blank_row}\r");

        let player_str = self.get_player_string(width);
        println!("{player_str}\r");
        print!("{blank_row}");
    }

    fn get_player_string(&self, width: usize) -> String {
        let left_width = width / 3;
        let right_width = width - left_width;

        let playing_msg = if self.state.playing {
            " [playing] "
        } else {
            " [paused]  "
        };
        let row_left = format!(
            "{playing_msg}{}", 
            " ".to_string().repeat(left_width - playing_msg.len())
        );

        let progress_str = format!(" {} ", get_duration_string(&self.state.song_progress));
        let length_str = format!(" {} ", get_duration_string(&self.state.song_length));

        let bar_total_width = right_width - progress_str.len() - length_str.len();
        let bar_progress_width = (bar_total_width as f32 * self.state.get_progress_val()) as usize;
        let bar_remainder_width = bar_total_width - bar_progress_width;

        let mut bar_progress_str = "=".to_string().repeat(bar_progress_width.saturating_sub(1));
        if bar_progress_str.len() != bar_progress_width {
            bar_progress_str.push('>');
        }
        let bar_remainder_str = "-".to_string().repeat(bar_remainder_width);

        format!(
            "{fg1}{row_left}{progress_str}{bar_progress_str}{fg2}{bar_remainder_str}{fg1}{length_str}",
            fg1 = color::Fg(PLAYED_COLOR),
            fg2 = color::Fg(UNPLAYED_COLOR)
        )
    }

    fn handle_keys(&mut self) -> Result<(), io::Error> {
        while let Ok(key) = self.terminal.relayer().rx.try_recv() {
            match key {
                QUIT_KEY => {
                    self.quit_requested = true;
                    break;
                },
                Key::Char(' ') => self.state.toggle_playing(),
                _ => (),
            }
        }
        Ok(())
    }
}

fn die(e: &io::Error) {
    Terminal::clear_screen();
    panic!("{e}");
}

fn get_duration_string(dur: &Duration) -> String {
    let secs = dur.as_secs();
    let mins = secs / 60;
    let secs = secs % 60;

    let mins_str = if mins < 10 {
        format!("0{mins}")
    } else {
        format!("{mins}")
    };
    let secs_str = if secs < 10 {
        format!("0{secs}")
    } else {
        format!("{secs}")
    };

    format!("{mins_str}:{secs_str}")
}