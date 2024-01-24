use gtk::glib::PropertyGet;
use gtk::{prelude::*, CustomFilter, Filter, ListView, PositionType, Widget};
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

    scrolled_window.connect_edge_reached(move | mut scroll, position| {
        // println!("{}", scroll.vadjustment().upper());
        if PositionType::Bottom == position {
            println!("No fim da lista");
            let list_box_clone = list_box.clone();
            for number in 0..=100 {
                let mut label_name = String::from("label = ");
                label_name.push_str(&number.to_string());
                let label = Label::new(Some(&label_name));
                list_box_clone.append(&label);
            }
            scroll.set_child(Option::Some(&list_box_clone));
        }
        
        if PositionType::Top == position {
            println!("No inicio da lista");
        }
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("list app")
        .default_width(600)
        .default_height(300)
        .child(&scrolled_window)
        .build();
    window.present();
}
