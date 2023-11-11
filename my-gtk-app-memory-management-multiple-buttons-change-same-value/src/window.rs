use std::cell::Cell;
use std::rc::Rc;
use gtk::{prelude::*, Button, Application, ApplicationWindow, Orientation};

pub struct Wind {
    window: ApplicationWindow
}

impl Wind {

    pub fn new(app: &Application) -> Self {
        Self {
            window: ApplicationWindow::builder()
                .title("testes")
                .application(app)
                .build()
        }
    }

    pub fn set_buttons(&self) {
        let button_increase = Button::builder()
            .label("Increase")
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();
        let button_decrease = Button::builder()
            .label("Decrease")
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();
        let number = Rc::new(Cell::new(0));
        let number_copy = number.clone();
        button_increase.connect_clicked(move |_| number_copy.set(number_copy.get() + 1));
        button_decrease.connect_clicked(move |_| number.set(number.get() - 1));

        let gtk_box = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .build();
        gtk_box.append(&button_decrease);
        gtk_box.append(&button_increase);
        self.window.set_child(Option::Some(&gtk_box));
    }

    pub fn present(&self) {
        self.window.present();
    }
}