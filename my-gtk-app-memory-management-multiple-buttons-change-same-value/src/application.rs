use gtk::{Application, glib};
use gtk::prelude::{ApplicationExt, ApplicationExtManual};
use crate::window::Wind;

pub struct App {
}

impl App {
    const NAME: &'static str = "SonecaApp";

    pub fn new() -> glib::ExitCode {
        let app = Application::builder()
            .application_id(Self::NAME)
            .build();
        app.connect_activate(|app| {
            let mut  wind = Wind::new(app);
            wind.set_buttons();
            wind.present();
        });
        app.run()
    }
}
