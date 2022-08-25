use std::io::stdin;
use std::sync::mpsc;
use std::thread::{self, JoinHandle};
use termion::event::Key;
use termion::input::TermRead;

use crate::QUIT_KEY;

pub struct Relayer {
    pub rx: mpsc::Receiver<Key>,
    _handle: JoinHandle<()>,
}

impl Relayer {
    pub fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        let _handle = thread::spawn(move || {
            Relayer::relay_input(tx);
        });
        Self { rx, _handle }
    }

    fn relay_input(tx: mpsc::Sender<Key>) {
        loop {
            if let Some(Ok(key)) = stdin().lock().keys().next() {
                tx.send(key).unwrap();
                if key == QUIT_KEY {
                    break;
                }
            }
        }
    }
}
