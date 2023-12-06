use gtk::{prelude::*, glib, gio, Application, Switch, Align, ApplicationWindow};
use gio::Settings;


const APP_ID: &str = "org.gtk_rs.Settings1";
const SWITCH_NAME: &str = "is-switch-enabled";

fn main() -> glib::ExitCode {
    // Command to move settings to glib/schemas
    /*
        mkdir -p $HOME/.local/share/glib-2.0/schemas
        cp org.gtk_rs.Settings2.gschema.xml $HOME/.local/share/glib-2.0/schemas/
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
    let switch = Switch::builder()
        .margin_bottom(12)
        .margin_end(32)
        .margin_start(11)
        .margin_top(22)
        .valign(Align::Center)
        .halign(Align::Center)
        .build();

    settings
        .bind(SWITCH_NAME, &switch, "active")
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My gtk app")
        .child(&switch)
        .build();

    window.present();
}
