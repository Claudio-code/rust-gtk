use std::cell::Cell;
use::gtk::{prelude::*, Button};
use::gtk::{glib, Application, ApplicationWindow};

const APP_ID: &str = "org.cia.soneca";

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("Memory safe")
        .margin_bottom(12)
        .margin_end(12)
        .margin_start(12)
        .margin_top(12)
        .build();

    // cell copy value to another reference
    let mut number = Cell::new(2);

    // when a button is clicked, number should be changed
    button.connect_clicked(move |_| number.set(number.get() + 1));

    // create window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Memory leak")
        .child(&button)
        .build();

    // present window
    window.present();
}
