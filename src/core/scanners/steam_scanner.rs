use crate::core::models::{Game, GamePlatform};
use directories::UserDirs;
use log::debug;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub fn scan() -> Vec<Game> {
    let mut games_map: HashMap<String, Game> = HashMap::new();
    let install_paths = get_steam_install_paths();

    if install_paths.is_empty() {
        debug!("No Steam installation found.");
        return vec![];
    }

    for base_path in install_paths {
        let library_folders = get_library_folders(&base_path);

        for library_path in library_folders {
            let steamapps_path = library_path.join("steamapps");
            if !steamapps_path.exists() {
                continue;
            }

            if let Ok(entries) = fs::read_dir(&steamapps_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    let file_name = path.file_name().unwrap_or_default().to_string_lossy();

                    if file_name.starts_with("appmanifest_") && file_name.ends_with(".acf") {
                        if let Some(game) = parse_manifest(&path, &steamapps_path) {
                            if !is_steam_tool(&game.name) && Path::new(&game.install_path).exists() {
                                games_map.insert(game.app_id.clone(), game);
                            }
                        }
                    }
                }
            }
        }
    }

    games_map.into_values().collect()
}

fn is_steam_tool(name: &str) -> bool {
    let lower = name.to_lowercase();
    lower.contains("proton")
        || lower.contains("steam linux runtime")
        || lower.contains("steamworks shared")
        || lower.contains("common redistributables")
        || lower.contains("steam client")
        || lower.contains("sdk")
}

fn get_steam_install_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if let Some(user_dirs) = UserDirs::new() {
        let home = user_dirs.home_dir();

        let native_path = home.join(".local").join("share").join("Steam");
        if native_path.exists() {
            paths.push(native_path);
        }

        let alt_native_path = home.join(".steam").join("root");
        if alt_native_path.exists() && !paths.contains(&alt_native_path) {
            paths.push(alt_native_path);
        }

        let flatpak_paths = vec![
            home.join(".var/app/com.valvesoftware.Steam/.local/share/Steam"),
            home.join(".var/app/com.valvesoftware.Steam/data/Steam"),
        ];

        for path in flatpak_paths {
            if path.exists() && !paths.contains(&path) {
                paths.push(path);
            }
        }
    }

    paths
}

fn get_library_folders(steam_path: &Path) -> Vec<PathBuf> {
    let mut folders = vec![steam_path.to_path_buf()];
    let vdf_path = steam_path.join("steamapps/libraryfolders.vdf");

    if let Ok(content) = fs::read_to_string(&vdf_path) {
        if let Ok(re) = Regex::new(r#""path"\s+"([^"]+)""#) {
            for cap in re.captures_iter(&content) {
                if let Some(path_match) = cap.get(1) {
                    let path_str = path_match.as_str().replace("\\\\", "/").replace("\\", "/");
                    let path = PathBuf::from(path_str);
                    if !folders.contains(&path) {
                        folders.push(path);
                    }
                }
            }
        }
    }

    folders
}

fn parse_manifest(manifest_path: &Path, steamapps_path: &Path) -> Option<Game> {
    let content = fs::read_to_string(manifest_path).ok()?;

    let app_id = Regex::new(r#""appid"\s+"(\d+)""#).ok()?
        .captures(&content)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
        .unwrap_or_else(|| {
            manifest_path.file_name().unwrap_or_default().to_string_lossy()
                .replace("appmanifest_", "")
                .replace(".acf", "")
        });

    let name = Regex::new(r#""name"\s+"([^"]+)""#).ok()?
        .captures(&content)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
        .unwrap_or_else(|| "Unknown Game".to_string());

    let install_dir = Regex::new(r#"("installdir"|"InstallDir")\s+"([^"]+)""#).ok()?
        .captures(&content)
        .and_then(|cap| cap.get(2))
        .map(|m| m.as_str().to_string())?;

    let full_install_path = steamapps_path.join("common").join(install_dir)
        .to_string_lossy().to_string();

    Some(Game {
        app_id,
        name,
        install_path: full_install_path,
        executable_path: None,
        platform: GamePlatform::Steam,
    })
}
