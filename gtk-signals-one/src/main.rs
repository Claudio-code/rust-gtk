use glib::closure_local;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};

const APP_ID: &str = "org.dream";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("Press me!")
        .margin_end(22)
        .margin_top(22)
        .margin_start(22)
        .margin_bottom(22)
        .build();

    // anchor callback
    button.connect_closure(
        "clicked",
        false,
        closure_local!(move |button: Button| {
            button.set_label("Hello word again!!");
        }),
    );

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Test")
        .child(&button)
        .build();

    window.present();
}
