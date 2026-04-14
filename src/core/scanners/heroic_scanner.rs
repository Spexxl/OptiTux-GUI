use crate::core::models::{Game, GamePlatform};
use directories::UserDirs;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const MIN_EXE_SIZE_BYTES: u64 = 1024 * 1024;

#[derive(Deserialize)]
struct HeroicGame {
    #[serde(rename = "app_name")]
    app_name: String,
    title: String,
    #[serde(rename = "install_path")]
    install_path: String,
    executable: Option<String>,
}

pub fn scan() -> Vec<Game> {
    let mut games = Vec::new();
    let config_paths = get_heroic_config_paths();

    for config_path in config_paths {
        if !config_path.exists() { continue; }

        if let Ok(content) = fs::read_to_string(&config_path) {
            if let Ok(data) = serde_json::from_str::<HashMap<String, HeroicGame>>(&content) {
                for (_, hg) in data {
                    let mut exec_path = None;
                    
                    if let Some(rel_exec) = hg.executable {
                        let full_path = Path::new(&hg.install_path).join(rel_exec);
                        if full_path.exists() {
                            exec_path = Some(full_path.to_string_lossy().to_string());
                        }
                    }

                    if exec_path.is_none() {
                        exec_path = find_best_executable(Path::new(&hg.install_path));
                    }

                    games.push(Game {
                        app_id: hg.app_name,
                        name: hg.title,
                        install_path: hg.install_path,
                        executable_path: exec_path,
                        platform: GamePlatform::Heroic,
                    });
                }
            }
        }
    }
    games
}

fn find_best_executable(root: &Path) -> Option<String> {
    if !root.exists() { return None; }
    let mut best_exe = None;

    for entry in WalkDir::new(root)
        .max_depth(3)
        .into_iter()
        .flatten()
    {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext.to_string_lossy().to_lowercase() == "exe" {
                    let file_name = path.file_stem().unwrap_or_default().to_string_lossy();
                    if is_trash_executable(&file_name) { continue; }

                    if let Ok(metadata) = entry.metadata() {
                        if metadata.len() < MIN_EXE_SIZE_BYTES { continue; }
                        
                        if let Some(parent) = path.parent() {
                            if has_upscaler_dll_nearby(parent) {
                                return Some(path.to_string_lossy().to_string());
                            }
                        }
                        if best_exe.is_none() {
                            best_exe = Some(path.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }
    }
    best_exe
}

fn has_upscaler_dll_nearby(dir: &Path) -> bool {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let file_name = entry.file_name().to_string_lossy().to_lowercase();
            if file_name == "nvngx.dll" || file_name == "libxess.dll" || file_name.contains("ffx_fsr") {
                return true;
            }
        }
    }
    false
}

fn is_trash_executable(name: &str) -> bool {
    let lower = name.to_lowercase();
    lower.contains("unins") || lower.contains("setup") || lower.contains("install") || 
    lower.contains("crash") || (lower.contains("launcher") && !lower.contains("game"))
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
