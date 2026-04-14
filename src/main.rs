mod core;

use env_logger;
use log::info;

fn main() {
    env_logger::init();
    
    info!("--- TESTING STEAM SCANNER WITH AUTO-EXE DETECTION ---");
    let steam_games = core::scanners::steam_scanner::scan();
    for game in steam_games.iter().take(5) {
        println!("Game: {} | Exe: {}", game.name, game.executable_path.as_deref().unwrap_or("NOT FOUND"));
    }

    info!("--- TESTING HEROIC SCANNER WITH AUTO-EXE DETECTION ---");
    let heroic_games = core::scanners::heroic_scanner::scan();
    for game in heroic_games {
        println!("Game: {} | Exe: {}", game.name, game.executable_path.as_deref().unwrap_or("NOT FOUND"));
    }
}
