mod app;
mod terminal;
mod sptfy;

pub use app::QUIT_KEY;
pub use terminal::{Terminal, Position};
pub use sptfy::Sptfy;

use app::App;

fn main() {
    App::default().run();
}
