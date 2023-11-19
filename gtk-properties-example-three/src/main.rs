mod custom_button;

use gtk::prelude::ApplicationExtManual;
use gtk::prelude::*;
use gtk::{glib, Align, Application, ApplicationWindow, Box, Orientation};
use crate::custom_button::CustomButton;

const APP_ID: &str = "org.gtk_rs.soneca";

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let button_one = CustomButton::new();
    let button_two = CustomButton::new();

    button_one
        .bind_property("number", &button_two, "number")
        .transform_to(|_, number: i32| {
            let increment_number = number + 1;
            Some(increment_number.to_value())
        })
        .transform_from(|_, number: i32| {
            let decremented_number = number - 1;
            Some(decremented_number.to_value())
        })
        .bidirectional()
        .sync_create()
        .build();

    button_one.connect_number_notify(|button| {
        println!("current number of `button_one` is {}", button.number());
    });

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
    gtk_box.append(&button_one);
    gtk_box.append(&button_two);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Finished")
        .child(&gtk_box)
        .build();
    window.present();
}
