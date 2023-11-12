mod custom_button;

use custom_button::CustomButton;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
use gtk::glib::PropertyGet;

const APP_ID: &str = "org.soneca.sub";
const LABEL_NO_CLICKED: &str = "baby";
const LABEL_CLICKED: &str = "hello word";

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let button = CustomButton::with_label(LABEL_NO_CLICKED);
    button.set_margin_end(21);
    button.set_margin_bottom(21);
    button.set_margin_start(21);
    button.set_margin_top(21);


    button.connect_clicked(move |button_internal| {
        let label = button_internal.label().unwrap().as_str().to_owned();
        if String::eq(&label, LABEL_CLICKED) {
            button_internal.set_label(LABEL_NO_CLICKED);
            return;
        }
        button_internal.set_label(LABEL_CLICKED);
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("App m")
        .child(&button)
        .build();
    window.present();
}
