use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

pub fn on_activate(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("OptiTux-GUI")
        .default_width(800)
        .default_height(600)
        .build();

    window.present();
}
