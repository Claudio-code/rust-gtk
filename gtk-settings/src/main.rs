use gtk::{prelude::*, glib, gio, Application, Switch, Align, ApplicationWindow};
use gio::Settings;

const APP_ID: &str = "org.gtk_rs.Settings1";
const SWITCH_NAME: &str = "is-switch-enabled";

fn main() -> glib::ExitCode {
    // Command to move settings to glib/schemas
    /*
        mkdir -p $HOME/.local/share/glib-2.0/schemas
        cp org.gtk_rs.Settings1.gschema.xml $HOME/.local/share/glib-2.0/schemas/
        glib-compile-schemas $HOME/.local/share/glib-2.0/schemas/
     */

     let app = Application::builder()
        .application_id(APP_ID)
        .build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let settings = Settings::new(APP_ID);
    let is_switch_enabled = settings.boolean(SWITCH_NAME);
    let switch = Switch::builder()
        .margin_bottom(12)
        .margin_end(32)
        .margin_start(11)
        .margin_top(22)
        .valign(Align::Center)
        .halign(Align::Center)
        .state(is_switch_enabled)
        .build();

    switch.connect_state_set(move |_, is_enabled| {
        settings
            .set_boolean(SWITCH_NAME, is_enabled)
            .expect("Could not set settings.");
        glib::Propagation::Proceed
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My gtk app")
        .child(&switch)
        .build();

    window.present();
}
