use std::thread;
use std::time::Duration;
use gtk::prelude::*;
use gtk::{Application, glib, gio, Button, ApplicationWindow};
use glib::clone;
use async_channel::{bounded, TryRecvError, TrySendError};


const APP_ID: &str = "org.teste";

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .margin_end(22)
        .margin_bottom(22)
        .margin_top(11)
        .margin_start(11)
        .label("Test async")
        .build();

    // create channel that can hold at most 1 message at a time
    let (sender, receiver) = bounded::<bool>(1);

    // connect to `clicked` signal of `button`
    button.connect_clicked(move |_| {
        let sender_clone = sender.clone();
        // long running operation runs now in a separate thread
        gio::spawn_blocking(move || {
            // deactivate button until the operation is done
            sender_clone
                .send_blocking(false)
                .expect("Error send one message.");

            let five_seconds = Duration::from_secs(5);
            thread::sleep(five_seconds);

            // activate button again
            sender_clone
                .send_blocking(true)
                .expect("Error when try send second message.");
        });
    });

    glib::spawn_future_local(clone!(@weak button => async move {
        while let Ok(enable_button) = receiver.recv().await {
            button.set_sensitive(enable_button);
        }
    }));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("async test")
        .child(&button)
        .build();
    window.present();
}

