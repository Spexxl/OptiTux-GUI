use anyhow::{anyhow, Result};
use directories::ProjectDirs;
use std::fs;
use std::path::{Path, PathBuf};

use crate::core::optiscaler::github::{Asset, GitHubClient, Release};

pub struct OptiScalerManager;

impl OptiScalerManager {
    fn versions_dir() -> Option<PathBuf> {
        if let Some(proj_dirs) = ProjectDirs::from("com", "OptiTux", "OptiTux") {
            let data_dir = proj_dirs.data_dir().join("versions");
            if !data_dir.exists() {
                let _ = fs::create_dir_all(&data_dir);
            }
            Some(data_dir)
        } else {
            None
        }
    }

    pub async fn get_available_online() -> Result<Vec<Release>> {
        GitHubClient::get_latest_releases().await
    }

    pub fn get_downloaded_versions() -> Vec<String> {
        let mut versions = Vec::new();
        
        if let Some(dir) = Self::versions_dir() {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        let file_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                        if file_name.ends_with(".zip") || file_name.ends_with(".7z") {
                            versions.push(file_name);
                        }
                    }
                }
            }
        }
        
        versions.sort();
        versions.reverse();
        
        versions
    }

    pub async fn download_version(asset: &Asset) -> Result<PathBuf> {
        if let Some(dir) = Self::versions_dir() {
            GitHubClient::download_asset(asset, &dir).await
        } else {
            Err(anyhow!("Could not determine local data directory to download OptiScaler versions."))
        }
    }

    pub fn remove_downloaded_version(file_name: &str) -> Result<()> {
        if let Some(dir) = Self::versions_dir() {
            let file_path = dir.join(file_name);
            if file_path.exists() {
                fs::remove_file(file_path)?;
            }
            Ok(())
        } else {
            Err(anyhow!("Could not determine local data directory."))
        }
    }

    pub fn get_version_path(file_name: &str) -> Option<PathBuf> {
        if let Some(dir) = Self::versions_dir() {
            let file_path = dir.join(file_name);
            if file_path.exists() {
                return Some(file_path);
            }
        }
        None
    }
}
