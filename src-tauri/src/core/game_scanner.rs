use crate::core::models::Game;
use crate::core::scanners::{steam_scanner, heroic_scanner, lutris_scanner, manual_scanner};
use crate::core::cache::GameCache;

pub struct ScannerManager;

impl ScannerManager {
    pub fn get_games(force_rescan: bool, custom_folders: &[String]) -> Vec<Game> {
        if !force_rescan {
            let cached = GameCache::load();
            if !cached.is_empty() {
                return cached;
            }
        }

        let mut all_games = Vec::new();

        all_games.append(&mut steam_scanner::scan());
        all_games.append(&mut heroic_scanner::scan());
        all_games.append(&mut lutris_scanner::scan());

        for folder in custom_folders {
            all_games.append(&mut manual_scanner::scan(folder));
        }

        all_games.retain(|g| g.executable_path.is_some());

        GameCache::save(&all_games);
        
        all_games
    }
}
