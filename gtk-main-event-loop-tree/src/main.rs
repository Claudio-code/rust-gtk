use ashpd::WindowIdentifier;
use ashpd::desktop::account::UserInformation;
use gtk::glib::clone;
use gtk::{glib, Application, Button, ApplicationWindow};
use gtk::prelude::*;

const APP_ID: &str = "org.soneca.test";

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("bye")
        .margin_bottom(12)
        .margin_end(12)
        .margin_start(12)
        .margin_top(12)
        .tooltip_text(" hey hello word")
        .build();
    
    button.connect_clicked(move |button| {
        glib::spawn_future_local(clone!(@weak button => async move {
            let native = button.native().expect("Need to be able to get native");
            let identifier = WindowIdentifier::from_native(&native).await;
            let request = UserInformation::request()
                .reason("App would like to access user infromation.")
                .identifier(identifier)
                .send()
                .await;
            if let Ok(response) = request.and_then(|r| r.response()) {
                println!("User name: {}", response.name());
            } else {
                println!("Could not access user information.")
            }
        }));
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("my app")
        .child(&button)
        .build();
    window.present();
}
