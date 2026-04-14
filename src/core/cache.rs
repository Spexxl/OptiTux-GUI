use crate::core::models::Game;
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

pub struct GameCache;

impl GameCache {
    fn cache_path() -> Option<PathBuf> {
        if let Some(proj_dirs) = ProjectDirs::from("com", "OptiTux", "OptiTux") {
            let config_dir = proj_dirs.config_dir();
            if !config_dir.exists() {
                let _ = fs::create_dir_all(config_dir);
            }
            Some(config_dir.join("games_cache.json"))
        } else {
            None
        }
    }

    pub fn load() -> Vec<Game> {
        if let Some(path) = Self::cache_path() {
            if path.exists() {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(games) = serde_json::from_str::<Vec<Game>>(&content) {
                        return games;
                    }
                }
            }
        }
        Vec::new()
    }

    pub fn save(games: &[Game]) {
        if let Some(path) = Self::cache_path() {
            if let Ok(json) = serde_json::to_string_pretty(games) {
                let _ = fs::write(path, json);
            }
        }
    }
}
