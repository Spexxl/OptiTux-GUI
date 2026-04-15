use crate::core::models::{Game, GamePlatform};
use directories::UserDirs;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
struct LutrisGameConfig {
    game: Option<LutrisGameDetails>,
}

#[derive(Deserialize)]
struct LutrisGameDetails {
    exe: Option<String>,
}

pub fn scan() -> Vec<Game> {
    let mut games = Vec::new();
    let config_paths = get_lutris_config_paths();

    for config_path in config_paths {
        let games_dir = config_path.join("games");
        if !games_dir.exists() {
            continue;
        }

        if let Ok(entries) = fs::read_dir(games_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("yml") {
                    if let Some(game) = parse_lutris_yaml(&path) {
                        games.push(game);
                    }
                }
            }
        }
    }

    games
}

fn get_lutris_config_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if let Some(user_dirs) = UserDirs::new() {
        let home = user_dirs.home_dir();

        paths.push(home.join(".local/share/lutris"));
        paths.push(home.join(".var/app/net.lutris.Lutris/data/lutris"));
    }

    paths
}

fn parse_lutris_yaml(path: &Path) -> Option<Game> {
    let content = fs::read_to_string(path).ok()?;
    let config: LutrisGameConfig = serde_yaml::from_str(&content).ok()?;

    let exe_path_str = config.game?.exe?;
    let exe_path = Path::new(&exe_path_str);
    
    let install_path = if let Some(parent) = exe_path.parent() {
        parent.to_string_lossy().to_string()
    } else {
        exe_path_str.clone()
    };

    let filename = path.file_stem()?.to_string_lossy().to_string();
    let mut name_parts: Vec<&str> = filename.split('-').collect();
    if name_parts.len() > 1 {
        name_parts.pop(); 
    }
    
    let display_name = name_parts.join(" ")
        .split_whitespace()
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ");

    let mut upscalars = Vec::new();
    if let Some(parent) = exe_path.parent() {
        upscalars = get_upscalers_nearby(parent);
    }

    Some(Game {
        app_id: filename.clone(),
        name: if display_name.is_empty() { filename } else { display_name },
        install_path,
        executable_path: Some(exe_path_str),
        upscalars,
        platform: GamePlatform::Lutris,
        cover_url: None,
    })
}

fn get_upscalers_nearby(dir: &Path) -> Vec<String> {
    let mut upscalers = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let file_name = entry.file_name().to_string_lossy().to_lowercase();
            if file_name.contains("nvngx") {
                if !upscalers.contains(&"DLSS".to_string()) { upscalers.push("DLSS".to_string()); }
            }
            if file_name.contains("libxess") {
                if !upscalers.contains(&"XeSS".to_string()) { upscalers.push("XeSS".to_string()); }
            }
            if file_name.contains("ffx") || file_name.contains("fsr") || file_name.contains("fidelityfx") {
                if !upscalers.contains(&"FSR".to_string()) { upscalers.push("FSR".to_string()); }
            }
        }
    }
    upscalers
}
