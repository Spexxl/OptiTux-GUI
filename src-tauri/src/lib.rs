pub mod core;

use crate::core::config::ConfigManager;
use crate::core::game_scanner::ScannerManager;
use crate::core::gpu_detector::{GpuDetector, GpuInfo};
use crate::core::models::Game;
use crate::core::optiscaler::github::{GitHubClient, Release};
use crate::core::optiscaler::installer::{Installer, InjectionMethod, FGInput, FGOutput};
use crate::core::optiscaler::manager::OptiScalerManager;
use serde::Serialize;
use tauri::{AppHandle, Emitter};

#[tauri::command]
async fn scan_games(
    app: AppHandle,
    force_rescan: bool,
    mut custom_folders: Vec<String>,
) -> Vec<Game> {
    let config = ConfigManager::load(&app);
    if custom_folders.is_empty() {
        custom_folders = config.custom_folders.clone();
    }
    ScannerManager::get_games(force_rescan, &custom_folders, &config.custom_covers).await
}

#[tauri::command]
async fn fetch_auto_cover(game_name: String) -> Option<String> {
    crate::core::metadata::fetch_game_cover(&game_name).await
}

#[tauri::command]
async fn set_custom_cover(app: AppHandle, app_id: String, cover_url: String) -> Result<(), String> {
    let mut config = ConfigManager::load(&app);
    config.custom_covers.insert(app_id, cover_url);
    ConfigManager::save(&app, &config);
    Ok(())
}

#[tauri::command]
async fn save_cover_image(app_id: String, bytes: Vec<u8>, extension: String) -> Result<String, String> {
    use directories::ProjectDirs;

    let proj_dirs = ProjectDirs::from("com", "OptiTux", "OptiTux")
        .ok_or_else(|| "Could not determine config directory".to_string())?;

    let covers_dir = proj_dirs.config_dir().join("covers");
    std::fs::create_dir_all(&covers_dir)
        .map_err(|e: std::io::Error| e.to_string())?;

    let filename = format!("{}.{}", app_id, extension);
    let file_path = covers_dir.join(&filename);
    std::fs::write(&file_path, &bytes)
        .map_err(|e: std::io::Error| e.to_string())?;

    Ok(file_path.to_string_lossy().to_string())
}

#[tauri::command]
async fn remove_custom_cover(app: AppHandle, app_id: String) -> Result<(), String> {
    let mut config = ConfigManager::load(&app);
    config.custom_covers.remove(&app_id);
    ConfigManager::save(&app, &config);
    Ok(())
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

#[tauri::command]
async fn get_online_releases() -> Result<Vec<Release>, String> {
    GitHubClient::get_latest_releases()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_downloaded_versions() -> Vec<String> {
    OptiScalerManager::get_downloaded_versions()
}

#[tauri::command]
async fn remove_downloaded_version(folder_name: String) -> Result<(), String> {
    OptiScalerManager::remove_downloaded_version(&folder_name)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_fg_inputs() -> Vec<&'static str> {
    FGInput::all_str()
}

#[tauri::command]
fn get_fg_outputs() -> Vec<&'static str> {
    FGOutput::all_str()
}

#[tauri::command]
fn get_injection_methods() -> Vec<[&'static str; 2]> {
    InjectionMethod::all_pairs()
        .into_iter()
        .map(|(slug, filename)| [slug, filename])
        .collect()
}

#[tauri::command]
async fn open_versions_folder() -> Result<(), String> {
    let path = OptiScalerManager::versions_dir_pub()
        .ok_or_else(|| "Could not determine versions directory.".to_string())?;
    let path_str = path.to_string_lossy().to_string();

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&path_str)
            .env_remove("LD_LIBRARY_PATH")
            .spawn()
            .map_err(|e| e.to_string())?;
        Ok(())
    }
    #[cfg(not(target_os = "linux"))]
    {
        tauri_plugin_opener::open_path(path_str, None::<&str>).map_err(|e| e.to_string())
    }
}

#[tauri::command]
async fn open_game_folder(game: Game) -> Result<(), String> {
    let path_str = game.executable_path.unwrap_or(game.install_path);
    let path = std::path::Path::new(&path_str);
    let folder = if path.is_file() { path.parent().unwrap_or(path) } else { path };
    let folder_path = folder.to_string_lossy().to_string();

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&folder_path)
            .env_remove("LD_LIBRARY_PATH")
            .spawn()
            .map_err(|e| e.to_string())?;
        Ok(())
    }
    #[cfg(not(target_os = "linux"))]
    {
        tauri_plugin_opener::open_path(folder_path, None::<&str>).map_err(|e| e.to_string())
    }
}

#[tauri::command]
async fn open_url_cmd(url: String) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&url)
            .env_remove("LD_LIBRARY_PATH")
            .spawn()
            .map_err(|e| e.to_string())?;
        Ok(())
    }
    #[cfg(not(target_os = "linux"))]
    {
        tauri_plugin_opener::open_url(url, None::<&str>).map_err(|e| e.to_string())
    }
}

#[derive(Clone, Serialize)]
struct DownloadProgress {
    downloaded: u64,
    total: u64,
    percent: f64,
    phase: String,
}

#[tauri::command]
async fn download_optiscaler_version(
    app: AppHandle,
    _tag_name: String,
    asset_name: String,
    asset_url: String,
    asset_size: u64,
) -> Result<String, String> {
    use crate::core::optiscaler::github::Asset;
    use crate::core::optiscaler::manager::OptiScalerManager;

    let asset = Asset {
        name: asset_name,
        browser_download_url: asset_url,
        size: asset_size,
    };

    app.emit("download-progress", DownloadProgress {
        downloaded: 0,
        total: asset_size,
        percent: 0.0,
        phase: "downloading".to_string(),
    }).ok();

    let extract_dir = OptiScalerManager::download_and_extract(&asset)
        .await
        .map_err(|e| e.to_string())?;

    app.emit("download-progress", DownloadProgress {
        downloaded: asset_size,
        total: asset_size,
        percent: 50.0,
        phase: "checking_int8".to_string(),
    }).ok();

    let int8_already_present = OptiScalerManager::is_int8_present();

    if !int8_already_present {
        if let Ok(int8_asset) = GitHubClient::get_int8_addon().await {
            app.emit("download-progress", DownloadProgress {
                downloaded: 0,
                total: int8_asset.size,
                percent: 60.0,
                phase: "downloading_int8".to_string(),
            }).ok();

            let _ = OptiScalerManager::download_int8(&int8_asset).await;
        }
    }

    app.emit("download-progress", DownloadProgress {
        downloaded: asset_size,
        total: asset_size,
        percent: 100.0,
        phase: "done".to_string(),
    }).ok();

    Ok(extract_dir.to_string_lossy().to_string())
}

#[derive(Clone, Serialize)]
struct QuickInstallProgress {
    phase: String,
    percent: f64,
}

#[tauri::command]
async fn quick_install_optiscaler(app: AppHandle, game: Game) -> Result<(), String> {
    Installer::quick_install(&game, |phase, percent| {
        app.emit("quick-install-progress", QuickInstallProgress {
            phase: phase.to_string(),
            percent,
        }).ok();
    })
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
async fn custom_install_optiscaler(
    app: AppHandle,
    game: Game,
    version_folder: String,
    upscaler: String,
    install_int8: bool,
    enable_framegen: bool,
    is_mfg_version: bool,
    injection_method: String,
    fg_input: String,
    fg_output: String,
) -> Result<(), String> {
    let injection = InjectionMethod::from_str(&injection_method);
    Installer::custom_install(&game, &version_folder, &upscaler, install_int8, enable_framegen, is_mfg_version, injection, &fg_input, &fg_output, |phase, percent| {
        app.emit("custom-install-progress", QuickInstallProgress {
            phase: phase.to_string(),
            percent,
        }).ok();
    })
    .await
    .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = fix_path_env::fix();

    #[cfg(target_os = "linux")]
    {
        if std::env::var("WEBKIT_DISABLE_COMPOSITING_MODE").is_err() {
            std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
        }
        if std::env::var("WEBKIT_DISABLE_DMABUF_RENDERER").is_err() {
            std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
        }
    }

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
            uninstall_optiscaler,
            get_online_releases,
            get_downloaded_versions,
            remove_downloaded_version,
            open_versions_folder,
            open_game_folder,
            set_custom_cover,
            save_cover_image,
            remove_custom_cover,
            fetch_auto_cover,
            download_optiscaler_version,
            quick_install_optiscaler,
            custom_install_optiscaler,
            open_url_cmd,
            get_fg_inputs,
            get_fg_outputs,
            get_injection_methods,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
