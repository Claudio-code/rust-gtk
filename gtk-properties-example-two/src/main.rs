use gtk::prelude::*;
use gtk::{Switch, Box, Align, Orientation, ApplicationWindow, Application, glib};

const APP_ID: &str = "org.soneca.sub";

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let switch_one = Switch::new();
    let switch_two = Switch::new();

    switch_one
        .bind_property("active", &switch_two, "active")
        .bidirectional()
        .build();

    let gtk_box = Box::builder()
        .margin_top(22)
        .margin_end(22)
        .margin_bottom(22)
        .margin_start(22)
        .valign(Align::Center)
        .halign(Align::Center)
        .spacing(22)
        .orientation(Orientation::Horizontal)
        .build();
    gtk_box.append(&switch_two);
    gtk_box.append(&switch_one);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Another example")
        .child(&gtk_box)
        .build();
    window.present();
}
