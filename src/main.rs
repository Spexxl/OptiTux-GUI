slint::include_modules!();

mod core;

fn main() {
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    let _guard = rt.enter();

    let window = MainWindow::new().expect("Failed to create window");
    window.run().expect("Application error");
}
