use std::thread;
use std::time::Duration;

use gtk::{prelude::*, ApplicationWindow};
use gtk::{self, gio, glib, Application, Button};

const APP_IP: &str = "org.soneca.app";

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id(APP_IP)
        .build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("Press me please!!")
        .margin_bottom(32)
        .margin_end(32)
        .margin_start(32)
        .margin_top(32)
        .build();

    // blocking main thread    
    // button.connect_clicked(move |_| {
    //     let five_seconds = Duration::from_secs(5);
    //     thread::sleep(five_seconds);
    // });

    // simulating a time-consuming
    // task without crashing main event loop
    button.connect_clicked(move |_| {
        gio::spawn_blocking(move || {
            let five_seconds = Duration::from_secs(5);
            thread::sleep(five_seconds);
        });
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My app")
        .child(&button)
        .build();
    
    window.present();
}
