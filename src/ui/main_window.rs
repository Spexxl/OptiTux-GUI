slint::include_modules!();

pub fn run_ui() -> Result<(), slint::PlatformError> {
    let app = MainWindow::new()?;

    let app_weak = app.as_weak();
    app.on_request_increment(move || {
        let app = app_weak.unwrap();
        app.set_counter(app.get_counter() + 1);
    });

    let app_weak = app.as_weak();
    app.on_request_decrement(move || {
        let app = app_weak.unwrap();
        app.set_counter(app.get_counter() - 1);
    });

    app.run()
}
