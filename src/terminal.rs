mod relayer;

use std::io::{self, stdout, Write};
use eyre::Result;
use termion::input::TermRead;
use termion::event::Key;
use termion::raw::{RawTerminal, IntoRawMode};
use termion::color;

use relayer::Relayer;

pub struct Size {
    pub width: u16,
    pub height: u16,
}

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Terminal {
    size: Size,
    relayer: Relayer,
    _stdout: RawTerminal<io::Stdout>,
}

impl Terminal {

    pub fn default() -> Result<Self> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
            relayer: Relayer::default(),
            _stdout: stdout().into_raw_mode()?,
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn relayer(&self) -> &Relayer {
        &self.relayer
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    pub fn cursor_position(pos: &Position) {
        let Position { mut x, mut y } = pos;
        x = x.saturating_add(1);
        y = y.saturating_add(1);
        let x = x as u16;
        let y = y as u16;
        print!("{}", termion::cursor::Goto(x, y));
    }

    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }

    pub fn set_bg_color(color: color::Rgb) {
        print!("{}", color::Bg(color));
    }

    pub fn reset_bg_color() {
        print!("{}", color::Bg(color::Reset));
    }

    pub fn set_fg_color(color: color::Rgb) {
        print!("{}", color::Fg(color));
    }

    pub fn reset_fg_color() {
        print!("{}", color::Fg(color::Reset));
    }

    pub fn flush() -> Result<(), io::Error> {
        io::stdout().flush()
    }
}
