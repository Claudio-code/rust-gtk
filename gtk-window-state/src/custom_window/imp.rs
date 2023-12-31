use gio::Settings;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, ApplicationWindow};
use std::cell::OnceCell;

#[derive(Default)]
pub struct Window {
    pub settings: OnceCell<Settings>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "MyGtkAppWindow";
    type Type = super::Window;
    type ParentType = ApplicationWindow;
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        obj.setup_settings();
        obj.load_window_size();
    }
}

impl WidgetImpl for Window {}

impl WindowImpl for Window {
    fn close_request(&self) -> glib::Propagation {
        self.obj()
            .save_window_size()
            .expect("Failed to save window state.");
        glib::Propagation::Proceed
    }
}

impl ApplicationWindowImpl for Window {}
