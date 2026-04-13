use crate::core::models::{Game, GamePlatform};
use directories::UserDirs;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize)]
struct HeroicGame {
    #[serde(rename = "app_name")]
    app_name: String,
    title: String,
    #[serde(rename = "install_path")]
    install_path: String,
}

pub fn scan() -> Vec<Game> {
    let mut games = Vec::new();
    let config_paths = get_heroic_config_paths();

    for config_path in config_paths {
        if !config_path.exists() {
            continue;
        }

        if let Ok(content) = fs::read_to_string(&config_path) {
            if let Ok(data) = serde_json::from_str::<HashMap<String, HeroicGame>>(&content) {
                for (_, hg) in data {
                    games.push(Game {
                        app_id: hg.app_name,
                        name: hg.title,
                        install_path: hg.install_path,
                        executable_path: None,
                        platform: GamePlatform::Heroic,
                    });
                }
            }
        }
    }

    games
}

fn get_heroic_config_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if let Some(user_dirs) = UserDirs::new() {
        let home = user_dirs.home_dir();

        paths.push(home.join(".config/heroic/installed.json"));
        paths.push(home.join(".config/heroic/legendaryConfig/legendary/installed.json"));
        paths.push(home.join(".config/heroic/gog_store/installed.json"));
        paths.push(home.join(".var/app/com.heroicgameslauncher.hgl/config/heroic/installed.json"));
        paths.push(home.join(".var/app/com.heroicgameslauncher.hgl/config/heroic/legendaryConfig/legendary/installed.json"));
        paths.push(home.join(".var/app/com.heroicgameslauncher.hgl/config/heroic/gog_store/installed.json"));
    }

    paths
}
