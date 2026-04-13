use serde::Deserialize;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use anyhow::Result;

const GITHUB_API_URL: &str = "https://api.github.com/repos/cdoane/OptiScaler/releases";

#[derive(Debug, Deserialize, Clone)]
pub struct Release {
    pub tag_name: String,
    pub name: String,
    pub assets: Vec<Asset>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Asset {
    pub name: String,
    pub browser_download_url: String,
    pub size: u64,
}

pub struct OptiScalerManager;

impl OptiScalerManager {
    pub async fn get_latest_releases() -> Result<Vec<Release>> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("OptiTux-GUI"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        let response = client
            .get(GITHUB_API_URL)
            .send()
            .await?
            .json::<Vec<Release>>()
            .await?;

        Ok(response)
    }
}
