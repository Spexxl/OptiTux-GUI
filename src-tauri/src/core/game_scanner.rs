use crate::core::models::Game;
use crate::core::scanners::{steam_scanner, heroic_scanner, lutris_scanner, manual_scanner};
use crate::core::cache::GameCache;
use crate::core::metadata;
use futures::future::join_all;

pub struct ScannerManager;

impl ScannerManager {
    pub async fn get_games(force_rescan: bool, custom_folders: &[String]) -> Vec<Game> {
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

        // Fetch metadata (covers) in parallel
        let metadata_tasks: Vec<_> = all_games.iter().map(|game| {
            metadata::fetch_game_cover(&game.name)
        }).collect();

        let covers = join_all(metadata_tasks).await;

        for (game, cover) in all_games.iter_mut().zip(covers) {
            game.cover_url = cover;
        }

        GameCache::save(&all_games);
        
        all_games
    }
}
