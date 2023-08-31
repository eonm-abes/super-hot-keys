mod app;
mod config;
mod hotkeys;
mod macros;

use hotkeys::*;
use macros::*;

fn main() {
    app::SuperHotKeys::new().run();
}
