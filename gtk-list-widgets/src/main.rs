use gtk::prelude::*;
use gtk::{
    glib, Application, ApplicationWindow, Label, ListBox, PolicyType, ScrolledWindow,
};

const APP_ID: &str = "org.gtk_rs.ListWidgets1";

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let list_box = ListBox::new();
    for number in 0..=100 {
        let mut label_name = String::from("label = ");
        label_name.push_str(&number.to_string());
        let label = Label::new(Some(&label_name));

        list_box.append(&label);
    }

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .min_content_width(360)
        .child(&list_box)
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("list app")
        .default_width(600)
        .default_height(300)
        .child(&scrolled_window)
        .build();
    window.present();
}
