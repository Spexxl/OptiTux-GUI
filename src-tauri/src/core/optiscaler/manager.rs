use anyhow::{anyhow, Result};
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

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

    pub fn versions_dir_pub() -> Option<PathBuf> {
        Self::versions_dir()
    }

    fn addons_dir() -> Option<PathBuf> {
        if let Some(proj_dirs) = ProjectDirs::from("com", "OptiTux", "OptiTux") {
            let addons_dir = proj_dirs.data_dir().join("addons");
            if !addons_dir.exists() {
                let _ = fs::create_dir_all(&addons_dir);
            }
            Some(addons_dir)
        } else {
            None
        }
    }

    fn int8_path() -> Option<PathBuf> {
        Self::addons_dir().map(|d| d.join("amd_fidelityfx_upscaler_dx12.dll"))
    }

    pub fn is_int8_present() -> bool {
        Self::int8_path().map(|p| p.exists()).unwrap_or(false)
    }

    pub fn int8_path_pub() -> Option<PathBuf> {
        Self::int8_path()
    }

    pub async fn download_int8(asset: &Asset) -> Result<PathBuf> {
        let dir = Self::addons_dir()
            .ok_or_else(|| anyhow!("Could not determine local data directory."))?;
        let file_path = GitHubClient::download_asset(asset, &dir).await?;
        let int8_path = dir.join("amd_fidelityfx_upscaler_dx12.dll");
        if file_path != int8_path {
            fs::rename(&file_path, &int8_path)?;
        }
        Ok(int8_path)
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
                    if path.is_dir() {
                        versions.push(
                            path.file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string(),
                        );
                    }
                }
            }
        }

        versions.sort();
        versions.reverse();

        versions
    }

    pub async fn download_and_extract(asset: &Asset) -> Result<PathBuf> {
        let dir = Self::versions_dir()
            .ok_or_else(|| anyhow!("Could not determine local data directory."))?;

        let archive_path = GitHubClient::download_asset(asset, &dir).await?;
        let version_name = asset.name.replace(".zip", "").replace(".7z", "");
        let extract_dir = dir.join(&version_name);

        if !extract_dir.exists() {
            fs::create_dir_all(&extract_dir)?;
        }

        if let Some(ext) = archive_path.extension().and_then(|s| s.to_str()) {
            if ext == "7z" {
                sevenz_rust::decompress_file(&archive_path, &extract_dir)
                    .map_err(|e| anyhow!("Failed to extract 7z archive: {}", e))?;
            } else if ext == "zip" {
                let file = fs::File::open(&archive_path)?;
                let mut archive = zip::ZipArchive::new(file)?;
                archive.extract(&extract_dir)?;
            }
        }

        let _ = fs::remove_file(&archive_path);

        Ok(extract_dir)
    }

    pub fn remove_downloaded_version(folder_name: &str) -> Result<()> {
        let dir = Self::versions_dir()
            .ok_or_else(|| anyhow!("Could not determine local data directory."))?;

        let folder_path = dir.join(folder_name);
        if folder_path.exists() && folder_path.is_dir() {
            fs::remove_dir_all(folder_path)?;
        }
        Ok(())
    }

    pub fn get_version_path(folder_name: &str) -> Option<PathBuf> {
        if let Some(dir) = Self::versions_dir() {
            let folder_path = dir.join(folder_name);
            if folder_path.exists() && folder_path.is_dir() {
                return Some(folder_path);
            }
        }
        None
    }
}
