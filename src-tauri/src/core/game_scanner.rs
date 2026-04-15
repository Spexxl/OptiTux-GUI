use crate::core::models::Game;
use crate::core::scanners::{steam_scanner, heroic_scanner, lutris_scanner, manual_scanner};
use crate::core::metadata;
use crate::core::cache::GameCache;
use crate::core::optiscaler::installer::Installer;
use futures::future::join_all;

pub struct ScannerManager;

impl ScannerManager {
    pub async fn get_games(force_rescan: bool, custom_folders: &[String]) -> Vec<Game> {
        if !force_rescan {
            if let Some(cached_games) = GameCache::load() {
                if !cached_games.is_empty() {
                    return cached_games;
                }
            }
        }

        let mut games = Vec::new();

        games.extend(steam_scanner::scan());
        games.extend(heroic_scanner::scan());
        games.extend(lutris_scanner::scan());

        for folder in custom_folders {
            games.extend(manual_scanner::scan(folder));
        }

        for game in &mut games {
            game.is_optiscaler_installed = Installer::is_installed(game);
        }

        let mut metadata_tasks = Vec::new();
        for game in games {
            metadata_tasks.push(async move {
                let mut updated_game = game;
                if updated_game.cover_url.is_none() {
                    if let Ok(url) = metadata::get_game_cover(&updated_game.name).await {
                        updated_game.cover_url = Some(url);
                    }
                }
                updated_game
            });
        }

        let final_games = join_all(metadata_tasks).await;
        let _ = GameCache::save(&final_games);
        final_games
    }
}
