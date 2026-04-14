mod core;
mod ui;

use env_logger;
use log::info;

fn main() {
    env_logger::init();
    
    info!("Starting OptiTux-GUI");
    
    if let Err(e) = ui::main_window::run_ui() {
        log::error!("Failed to start UI: {}", e);
    }
}
