mod task_object;
mod task_row;
mod window;

use gtk::prelude::*;
use gtk::{gio, glib, Application};
use window::Window;

fn main() -> glib::ExitCode {
    gio::resources_register_include!("todo_1.gresource")
        .expect("Failed to register resources.");

    let app = Application::builder()
        .application_id("org.gtk_rs.Todo1")
        .build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let window = Window::new(app);
    window.present();
}