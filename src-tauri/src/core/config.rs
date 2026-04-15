use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub custom_folders: Vec<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            custom_folders: Vec::new(),
        }
    }
}

pub struct ConfigManager;

impl ConfigManager {
    fn get_config_path(app: &AppHandle) -> PathBuf {
        let mut path = app.path().app_config_dir().unwrap_or_else(|_| PathBuf::from("."));
        if !path.exists() {
            let _ = fs::create_dir_all(&path);
        }
        path.push("config.json");
        path
    }

    pub fn load(app: &AppHandle) -> AppConfig {
        let path = Self::get_config_path(app);
        if let Ok(content) = fs::read_to_string(path) {
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            AppConfig::default()
        }
    }

    pub fn save(app: &AppHandle, config: &AppConfig) {
        let path = Self::get_config_path(app);
        if let Ok(content) = serde_json::to_string_pretty(config) {
            let _ = fs::write(path, content);
        }
    }
}
