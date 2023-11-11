mod application;
mod window;

use gtk::{glib};
use self::application::App;

fn main() -> glib::ExitCode {
    println!("Hello, world!");
    App::new()
}
