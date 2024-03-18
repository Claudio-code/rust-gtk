mod task_object;
mod task_row;
mod utils;
mod window;

use gtk::prelude::*;
use gtk::{gio, glib, Application};
use window::Window;

const APP_ID: &str = "org.gtk_rs.Todo2";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("todo_2.gresource").expect("Failed to register resources.");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(setup_shortcuts);
    app.connect_activate(build_ui);
    app.run()
}

fn setup_shortcuts(app: &Application) {
    app.set_accels_for_action("win.filter('All')", &["<Ctrl>a"]);
    app.set_accels_for_action("win.filter('Open')", &["<Ctrl>o"]);
    app.set_accels_for_action("win.filter('Done')", &["<Ctrl>d"]);
}

fn build_ui(app: &Application) {
    let window = Window::new(app);
    window.present();
}
