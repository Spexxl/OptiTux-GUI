use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::Deserialize;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

const GITHUB_API_URL: &str = "https://api.github.com/repos/optiscaler/OptiScaler/releases";

#[derive(Debug, Deserialize, Clone)]
pub struct Release {
    pub tag_name: String,
    pub assets: Vec<Asset>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Asset {
    pub name: String,
    pub browser_download_url: String,
    pub size: u64,
}

pub struct GitHubClient;

impl GitHubClient {
    pub async fn get_latest_releases() -> Result<Vec<Release>> {
        let client = Client::builder()
            .user_agent("OptiTux-GUI")
            .build()?;

        let response = client.get(GITHUB_API_URL).send().await?;
        let releases = response.json::<Vec<Release>>().await?;
        
        Ok(releases)
    }

    pub async fn download_asset(asset: &Asset, target_dir: &Path) -> Result<PathBuf> {
        let client = Client::builder()
            .user_agent("OptiTux-GUI")
            .build()?;

        let response = client.get(&asset.browser_download_url).send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to download asset: HTTP {}", response.status()));
        }

        let bytes = response.bytes().await?;
        
        if !target_dir.exists() {
            fs::create_dir_all(target_dir)?;
        }

        let file_path = target_dir.join(&asset.name);
        let mut file = fs::File::create(&file_path)?;
        file.write_all(&bytes)?;

        Ok(file_path)
    }
}
