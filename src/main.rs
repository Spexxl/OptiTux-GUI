mod core;

use env_logger;
use log::info;

fn main() {
    env_logger::init();
    
    info!("Starting Steam Scanner test...");
    
    let games = core::scanners::steam_scanner::scan();
    
    if games.is_empty() {
        println!("No Steam games found.");
    } else {
        println!("Found {} Steam games:", games.len());
        for game in games {
            println!("---------------------------------");
            println!("Name: {}", game.name);
            println!("AppID: {}", game.app_id);
            println!("Install Path: {}", game.install_path);
        }
        println!("---------------------------------");
    }
}
