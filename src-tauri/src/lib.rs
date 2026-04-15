pub mod core;

use crate::core::game_scanner::ScannerManager;
use crate::core::models::Game;
use crate::core::gpu_detector::{GpuDetector, GpuInfo};

#[tauri::command]
async fn scan_games(force_rescan: bool, custom_folders: Vec<String>) -> Vec<Game> {
    ScannerManager::get_games(force_rescan, &custom_folders).await
}

#[tauri::command]
async fn get_gpu_info() -> Option<GpuInfo> {
    GpuDetector::detect_gpus().into_iter().find(|g| g.is_primary)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![scan_games, get_gpu_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
