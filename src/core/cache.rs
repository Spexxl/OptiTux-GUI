use super::models::Game;
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

pub struct GameCache;

impl GameCache {
    fn cache_path() -> Option<PathBuf> {
        if let Some(proj_dirs) = ProjectDirs::from("io", "github", "optitux") {
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
        let Some(path) = Self::cache_path() else { return vec![]; };
        if !path.exists() { return vec![]; }

        if let Ok(content) = fs::read_to_string(&path) {
            serde_json::from_str(&content).unwrap_or_else(|_| vec![])
        } else {
            vec![]
        }
    }

    pub fn save(games: &[Game]) {
        let Some(path) = Self::cache_path() else { return; };
        if let Ok(json) = serde_json::to_string_pretty(games) {
            let _ = fs::write(path, json);
        }
    }
}
