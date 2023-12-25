mod custom_window;

use custom_window::Window;
use gtk::prelude::*;
use gtk::{glib, Application, Button};

const APP_ID: &str = "org.gtk_rs.SavingWindowState1";

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let window = Window::new(app);

    let button = Button::builder()
        .label("Press me!")
        .margin_bottom(12)
        .margin_end(12)
        .margin_start(12)
        .margin_top(12)
        .build();

    button.connect_clicked(move |activated_button| {
        activated_button.set_label("Merry Christmas");
    });
    window.set_child(Some(&button));
    window.present();
}
