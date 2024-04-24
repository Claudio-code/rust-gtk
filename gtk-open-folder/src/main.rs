use std::{
    borrow::Borrow, fs::{self, File}, io::{prelude::*, BufReader}, path::PathBuf, sync::OnceLock, thread, time::Duration
};

use gtk::{gio::ListModel, prelude::*, Button, glib::clone};
use gtk::{glib, Application, ApplicationWindow};
use tokio::{runtime::Runtime, time::error::Error};
use percentage::Percentage;

const APP_ID: &str = "org.soneca";

fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."))
}


fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let progress = gtk::ProgressBar::new();
    progress.set_show_text(false);
    progress.set_hexpand(true);

    let button = Button::builder()
        .label("Press me!}")
        .margin_top(22)
        .margin_bottom(22)
        .margin_end(22)
        .margin_start(22)
        .build();

    
    let container = gtk::Grid::new();
    container.attach(&progress, 0, 0, 1, 1);
    container.attach(&button, 0, 1, 1, 1);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Hello word")
        .child(&container)
        .build();

    let (sender, receiver) = async_channel::bounded::<PathBuf>(1);

    button.connect_clicked(clone!(@weak window, @strong sender => move |_| {
        let file_dialog = gtk::FileDialog::builder()
            .title("Escolha a pasta")
            .accept_label("escolher")
            .build();

        file_dialog.select_folder(Some(&window), gtk::gio::Cancellable::NONE, clone!(@strong sender => move |folder| {
            if let Ok(folder) = folder {
                runtime().spawn(clone!(@strong sender => async move {
                    let path = folder.path().unwrap().clone();
                    let _ = sender.send(path).await;
                }));

            }
        }));
    }));


    glib::spawn_future_local(clone!(@weak progress => async move {
        while let Ok(response) = receiver.recv().await {
            println!("file path received {:?}", response);
            let paths = fs::read_dir(response)
                .unwrap()
                .map(|entry| entry.unwrap().path().display().to_string())
                .collect::<Vec<String>>();

            // let percentage = Percentage::from(paths.len());
            let total = 100;
            let mut total_barra: f64 = 0.0;
            let porcenta = (total / paths.len()) as f64;

            for path in paths {
                total_barra += porcenta;
                println!("size received {:?}", total_barra);
                progress.set_fraction(total_barra);
            }
        }
    }));


    window.present()
}
