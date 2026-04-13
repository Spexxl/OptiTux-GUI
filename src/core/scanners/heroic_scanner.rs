use crate::core::models::{Game, GamePlatform};
use directories::UserDirs;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize)]
struct HeroicInstalled {
    installed: Vec<HeroicGame>,
}

#[derive(Deserialize)]
struct HeroicGame {
    #[serde(rename = "appName")]
    app_name: String,
    title: String,
    #[serde(rename = "installPath")]
    install_path: String,
}

pub fn scan() -> Vec<Game> {
    let mut games = Vec::new();
    let config_paths = get_heroic_config_paths();

    for config_path in config_paths {
        let installed_json = config_path.join("installed.json");
        if !installed_json.exists() {
            continue;
        }

        if let Ok(content) = fs::read_to_string(installed_json) {
            if let Ok(data) = serde_json::from_str::<HeroicInstalled>(&content) {
                for hg in data.installed {
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

        let native_path = home.join(".config").join("heroic");
        if native_path.exists() {
            paths.push(native_path);
        }

        let flatpak_path = home
            .join(".var")
            .join("app")
            .join("com.heroicgameslauncher.hgl")
            .join("config")
            .join("heroic");
        if flatpak_path.exists() {
            paths.push(flatpak_path);
        }
    }

    paths
}
