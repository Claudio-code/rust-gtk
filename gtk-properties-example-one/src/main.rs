use gtk::prelude::*;
use gtk::{Application, Switch, Box, Align, Orientation, ApplicationWindow, glib};

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.ui.soneca")
        .build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let switch = Switch::new();
    switch.set_active(true);

    let switch_active = switch.is_active();
    println!("active property of switch {}", switch_active);

    let gtk_box = Box::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .valign(Align::Center)
        .halign(Align::Center)
        .spacing(12)
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&switch);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("properties test")
        .child(&gtk_box)
        .build();
    window.present();
}
