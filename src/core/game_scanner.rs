use crate::core::models::Game;
use crate::core::cache::GameCache;
use crate::core::scanners::{steam_scanner, heroic_scanner, lutris_scanner, manual_scanner};
use log::{info, debug};

pub struct ScannerManager;

impl ScannerManager {
    pub fn get_games(force_rescan: bool, custom_folders: &[String]) -> Vec<Game> {
        if !force_rescan {
            let cached = GameCache::load();
            if !cached.is_empty() {
                debug!("Games loaded from cache");
                return cached;
            }
        }

        info!("Starting game scan...");

        let mut all_games = Vec::new();
        
        all_games.extend(steam_scanner::scan());
        all_games.extend(heroic_scanner::scan());
        all_games.extend(lutris_scanner::scan());

        for folder in custom_folders {
            all_games.extend(manual_scanner::scan(folder));
        }

        GameCache::save(&all_games);
        all_games
    }
}
