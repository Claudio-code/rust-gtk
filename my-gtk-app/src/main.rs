use gtk::{prelude::*, Button};
use gtk::{glib, Application, ApplicationWindow};

const APP_ID: &str = "org.soneca";

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("Press me!}")
        .margin_top(22)
        .margin_bottom(22)
        .margin_end(22)
        .margin_start(22)
        .build();

    let tooltip = Option::Some("tooltip text");
    button.set_tooltip_text(tooltip);
    button.connect_clicked(|button| {
        button.set_label("birulaibe");
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Hello word")
        .child(&button)
        .build();
    window.present()
}
