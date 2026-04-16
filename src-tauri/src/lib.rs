pub mod core;

use crate::core::config::ConfigManager;
use crate::core::game_scanner::ScannerManager;
use crate::core::gpu_detector::{GpuDetector, GpuInfo};
use crate::core::models::Game;
use tauri::AppHandle;

#[tauri::command]
async fn scan_games(
    app: AppHandle,
    force_rescan: bool,
    mut custom_folders: Vec<String>,
) -> Vec<Game> {
    if custom_folders.is_empty() {
        let config = ConfigManager::load(&app);
        custom_folders = config.custom_folders;
    }
    ScannerManager::get_games(force_rescan, &custom_folders).await
}

#[tauri::command]
async fn get_custom_folders(app: AppHandle) -> Vec<String> {
    let config = ConfigManager::load(&app);
    config.custom_folders
}

#[tauri::command]
async fn add_custom_folder(app: AppHandle, folder: String) -> Result<Vec<String>, String> {
    let mut config = ConfigManager::load(&app);
    if !config.custom_folders.contains(&folder) {
        config.custom_folders.push(folder);
        ConfigManager::save(&app, &config);
    }
    Ok(config.custom_folders)
}

#[tauri::command]
async fn remove_custom_folder(app: AppHandle, folder: String) -> Result<Vec<String>, String> {
    let mut config = ConfigManager::load(&app);
    config.custom_folders.retain(|f| f != &folder);
    ConfigManager::save(&app, &config);
    Ok(config.custom_folders)
}

#[tauri::command]
async fn get_gpu_info() -> Option<GpuInfo> {
    GpuDetector::detect_gpus().into_iter().find(|g| g.is_primary)
}

#[tauri::command]
async fn uninstall_optiscaler(game: Game) -> Result<(), String> {
    crate::core::optiscaler::installer::Installer::uninstall(&game)
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            scan_games,
            get_gpu_info,
            get_custom_folders,
            add_custom_folder,
            remove_custom_folder,
            uninstall_optiscaler
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
