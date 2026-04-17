use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

const GITHUB_API_URL_OFFICIAL: &str = "https://api.github.com/repos/optiscaler/OptiScaler/releases";
const GITHUB_API_URL_PRE_RELEASE: &str = "https://api.github.com/repos/Spexxl/OptiTux-Database/releases";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Release {
    pub tag_name: String,
    pub assets: Vec<Asset>,
    #[serde(default)]
    pub prerelease: bool,
    #[serde(default)]
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

        let mut all_releases = Vec::new();

        if let Ok(response) = client.get(GITHUB_API_URL_OFFICIAL).send().await {
            if let Ok(mut releases) = response.json::<Vec<Release>>().await {
                for r in &mut releases {
                    r.source = "stable".to_string();
                }
                all_releases.extend(releases);
            }
        }

        if let Ok(response) = client.get(GITHUB_API_URL_PRE_RELEASE).send().await {
            if let Ok(mut db_releases) = response.json::<Vec<Release>>().await {
                db_releases.retain(|r| r.tag_name != "INT8");
                for r in &mut db_releases {
                    r.source = "db".to_string();
                }
                all_releases.extend(db_releases);
            }
        }

        if all_releases.is_empty() {
            return Err(anyhow!("Failed to fetch releases from both Official and Database repositories. API limit or network issue."));
        }

        Ok(all_releases)
    }

    pub async fn get_int8_addon() -> Result<Asset> {
        let client = Client::builder()
            .user_agent("OptiTux-GUI")
            .build()?;

        let url = "https://api.github.com/repos/Spexxl/OptiTux-Database/releases/tags/INT8";
        let response = client.get(url).send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("INT8 release not found or rate limited."));
        }

        let release = response.json::<Release>().await?;
        
        if release.assets.is_empty() {
            return Err(anyhow!("No assets found in the INT8 release."));
        }

        Ok(release.assets[0].clone())
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
