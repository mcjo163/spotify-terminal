use std::time::{Duration, Instant};
use std::io;
use termion::event::Key;

use crate::{
    Terminal, 
    Position,
    Sptfy,
};

pub const QUIT_KEY: Key = Key::Ctrl('q');

pub struct App {
    quit_requested: bool,
    terminal: Terminal,
    _sptfy: Sptfy,
}

impl App {

    pub fn default() -> Self {
        App {
            quit_requested: false,
            terminal: Terminal::default().expect("failed to initialize terminal"),
            _sptfy: Sptfy::default(),
        }
    }

    pub fn run(&mut self) {
        loop {
            let now = Instant::now();
            if let Err(e) = self.render() {
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
        }
    }

    fn render(&self) -> Result<(), io::Error> {
        Terminal::cursor_position(&Position::default());
        Terminal::cursor_hide();

        Terminal::clear_screen();
        println!("rendering\r");

        if self.quit_requested {
            Terminal::clear_screen();
            Terminal::cursor_show();
        }

        Terminal::flush()
    }

    fn handle_keys(&mut self) -> Result<(), io::Error> {

        while let Ok(key) = self.terminal.relayer().rx.try_recv() {
            println!("{key:?}\r");
            match key {
                QUIT_KEY => self.quit_requested = true,
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