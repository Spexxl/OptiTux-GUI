mod core;

use env_logger;
use log::info;

fn main() {
    env_logger::init();
    
    info!("Starting Heroic Scanner test...");
    
    let games = core::scanners::heroic_scanner::scan();
    
    if games.is_empty() {
        println!("No Heroic games found.");
    } else {
        println!("Found {} Heroic games:", games.len());
        for game in games {
            println!("---------------------------------");
            println!("Name: {}", game.name);
            println!("AppID: {}", game.app_id);
            println!("Install Path: {}", game.install_path);
        }
        println!("---------------------------------");
    }
}
