use glib::clone;
use gtk::glib::once_cell::sync::Lazy;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};
use reqwest::Response;
use tokio::runtime::Runtime;

const APP_ID: &str = "org.soneca";
static RUNTIME: Lazy<Runtime> =
    Lazy::new(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."));


fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("Test")
        .margin_bottom(22)
        .margin_end(22)
        .margin_start(21)
        .margin_end(11)
        .build();

    let (sender, receiver) = async_channel::bounded::<Result<Response, reqwest::Error>>(1);

    button.connect_clicked(move |_| {
        // main loop executes the asynchronous block
        RUNTIME.spawn(clone!(@strong sender => async move {
            let response = reqwest::get("https://www.gtk-rs.org").await;
            sender.send(response).await.expect("The channel needs to be open.");
        }));
    });

    // main loop executes the asynchronous block
    glib::spawn_future_local(async move {
        while let Ok(response) = receiver.recv().await {
            if let Ok(response) = response {
                println!("status {}", response.status());
            } else {
                println!("bad request");
            }
        }
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("GTK app")
        .child(&button)
        .build();
    window.present();
}
