use crate::core::models::{Game, GamePlatform};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

const MIN_EXE_SIZE_BYTES: u64 = 248 * 248;

pub fn scan(folder_path: &str) -> Vec<Game> {
    let mut games = Vec::new();
    let root = Path::new(folder_path);

    if !root.exists() || !root.is_dir() {
        return games;
    }

    if is_game_folder(root) {
        if let Some(game) = scan_game_folder(root) {
            games.push(game);
            return games;
        }
    }

    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(game) = scan_game_folder(&path) {
                    games.push(game);
                }
            }
        }
    }

    games
}

fn is_game_folder(path: &Path) -> bool {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_file() {
                if let Some(ext) = p.extension() {
                    if ext.to_string_lossy().to_lowercase() == "exe" {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn scan_game_folder(game_folder: &Path) -> Option<Game> {
    let mut best_exe = None;

    for entry in WalkDir::new(game_folder)
        .max_depth(4)
        .into_iter()
        .flatten()
    {
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext.to_string_lossy().to_lowercase() == "exe" {
                    let file_name = path.file_stem().unwrap_or_default().to_string_lossy();

                    if is_trash_executable(&file_name) {
                        continue;
                    }

                    if let Ok(metadata) = entry.metadata() {
                        if metadata.len() < MIN_EXE_SIZE_BYTES {
                            continue;
                        }

                        if let Some(parent_dir) = path.parent() {
                            if has_upscaler_dll_nearby(parent_dir) {
                                best_exe = Some(path.to_path_buf());
                                break;
                            }
                        }

                        if best_exe.is_none() {
                            best_exe = Some(path.to_path_buf());
                        }
                    }
                }
            }
        }
    }

    if let Some(exe_path) = best_exe {
        let game_name = game_folder
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let install_path = exe_path
            .parent()
            .unwrap_or(game_folder)
            .to_string_lossy()
            .to_string();

        Some(Game {
            app_id: format!("Custom_{}", game_name),
            name: game_name,
            install_path,
            executable_path: Some(exe_path.to_string_lossy().to_string()),
            platform: GamePlatform::Custom,
        })
    } else {
        None
    }
}

fn has_upscaler_dll_nearby(dir: &Path) -> bool {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let file_name = entry.file_name().to_string_lossy().to_lowercase();
            
            if file_name == "nvngx.dll"
                || file_name == "libxess.dll"
                || file_name.contains("ffx_fsr2")
                || file_name.contains("ffx_fsr3")
            {
                return true;
            }
        }
    }
    false
}

fn is_trash_executable(name: &str) -> bool {
    let lower = name.to_lowercase();
    lower.contains("unins")
        || lower.contains("setup")
        || lower.contains("install")
        || lower.contains("crash")
        || (lower.contains("launcher") && !lower.contains("game"))
}
