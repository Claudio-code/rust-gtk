mod task_object;
mod task_row;
mod window;
mod utils;

use gtk::prelude::*;
use gtk::{gio, glib};
use window::Window;

const APP_ID: &str = "org.gtk_rs.Todo5";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("todo_5.gresource")
        .expect("Failed to register resources.");

    let app = adw::Application::builder().application_id(APP_ID).build();
    app.connect_startup(setup_shortcuts);
    app.connect_activate(build_ui);
    app.run()
}

fn setup_shortcuts(app: &adw::Application) {
    app.set_accels_for_action("win.filter('All')", &["<Ctrl>a"]);
    app.set_accels_for_action("win.filter('Open')", &["<Ctrl>o"]);
    app.set_accels_for_action("win.filter('Done')", &["<Ctrl>d"]);
}

fn build_ui(app: &adw::Application) {
    let window = Window::new(app);
    window.present();
}
