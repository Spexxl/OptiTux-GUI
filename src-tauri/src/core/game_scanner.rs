use crate::core::models::Game;
use crate::core::scanners::{steam_scanner, heroic_scanner, lutris_scanner, manual_scanner};
use crate::core::metadata;
use crate::core::cache::GameCache;
use crate::core::optiscaler::installer::Installer;
use futures::future::join_all;

use std::collections::HashMap;

pub struct ScannerManager;

impl ScannerManager {
    pub async fn get_games(force_rescan: bool, custom_folders: &[String], custom_covers: &HashMap<String, String>) -> Vec<Game> {
        if !force_rescan {
            let mut cached_games = GameCache::load();
            if !cached_games.is_empty() {
                for game in &mut cached_games {
                    game.is_optiscaler_installed = Installer::is_installed(game);
                    if let Some(custom_url) = custom_covers.get(&game.app_id) {
                        game.cover_url = Some(custom_url.clone());
                    }
                }
                return cached_games;
            }
        }

        let mut games = Vec::new();

        games.extend(steam_scanner::scan());
        games.extend(heroic_scanner::scan());
        games.extend(lutris_scanner::scan());

        for folder in custom_folders {
            games.extend(manual_scanner::scan(folder));
        }

        let mut unique_games = Vec::new();
        let mut seen_paths = std::collections::HashSet::new();

        for game in games {
            if let Some(ref path) = game.executable_path {
                if !seen_paths.contains(path) {
                    seen_paths.insert(path.clone());
                    unique_games.push(game);
                }
            } else {
                unique_games.push(game);
            }
        }
        let mut games = unique_games;

        for game in &mut games {
            game.is_optiscaler_installed = Installer::is_installed(game);
        }

        let mut metadata_tasks = Vec::new();
        for game in games {
            metadata_tasks.push(async move {
                let mut updated_game = game;
                if updated_game.cover_url.is_none() {
                    if let Some(url) = metadata::fetch_game_cover(&updated_game.name).await {
                        updated_game.cover_url = Some(url);
                    }
                }
                updated_game
            });
        }

        let mut final_games = join_all(metadata_tasks).await;
        
        let _ = GameCache::save(&final_games);

        for game in &mut final_games {
            if let Some(custom_url) = custom_covers.get(&game.app_id) {
                game.cover_url = Some(custom_url.clone());
            }
        }

        final_games
    }
}
