use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::Deserialize;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

const GITHUB_API_URL_OFFICIAL: &str = "https://api.github.com/repos/optiscaler/OptiScaler/releases";
const GITHUB_API_URL_PRE_RELEASE: &str = "https://api.github.com/repos/Spexxl/OptiTux-Database/releases";

#[derive(Debug, Deserialize, Clone)]
pub struct Release {
    pub tag_name: String,
    pub assets: Vec<Asset>,
    #[serde(default)]
    pub prerelease: bool,
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

        let mut all_releases = Vec::new();

        if let Ok(response) = client.get(GITHUB_API_URL_OFFICIAL).send().await {
            if let Ok(releases) = response.json::<Vec<Release>>().await {
                all_releases.extend(releases);
            }
        }

        if let Ok(response) = client.get(GITHUB_API_URL_PRE_RELEASE).send().await {
            if let Ok(mut db_releases) = response.json::<Vec<Release>>().await {
                for r in &mut db_releases {
                    r.prerelease = true;
                }
                all_releases.extend(db_releases);
            }
        }

        if all_releases.is_empty() {
            return Err(anyhow!("Failed to fetch releases from both Official and Database repositories. API limit or network issue."));
        }

        Ok(all_releases)
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
