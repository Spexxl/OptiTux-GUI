mod app;
mod config;
mod core;
mod ui;

use gtk::prelude::*;
use gtk::Application;

const APP_ID: &str = "io.github.optitux.gui";

fn main() {
    // We will initialize libadwaita/gtk here in the future.
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(app::on_activate);
    app.run();
}
