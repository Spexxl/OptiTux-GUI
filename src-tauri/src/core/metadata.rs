use reqwest;
use serde_json::Value;
use anyhow::{Result, anyhow};

pub async fn fetch_game_cover(game_name: &str) -> Option<String> {
    if let Ok(app_id) = search_steam_app_id(game_name).await {
        let cover_url = format!("https://cdn.cloudflare.steamstatic.com/steam/apps/{}/library_600x900.jpg", app_id);
        if verify_url(&cover_url).await {
            return Some(cover_url);
        }
    }
    None
}

async fn search_steam_app_id(game_name: &str) -> Result<u32> {
    let url = format!(
        "https://store.steampowered.com/api/storesearch/?term={}&l=english&cc=US",
        urlencoding::encode(game_name)
    );

    let client = reqwest::Client::builder()
        .user_agent("OptiTux-GUI")
        .build()?;

    let response = client.get(url).send().await?.json::<Value>().await?;

    if let Some(items) = response["items"].as_array() {
        if items.is_empty() {
            return Err(anyhow!("No results for: {}", game_name));
        }

        let search_name = game_name.to_lowercase();

        for item in items {
            if let Some(name) = item["name"].as_str() {
                if name.to_lowercase() == search_name {
                    if let Some(id) = item["id"].as_u64() {
                        return Ok(id as u32);
                    }
                }
            }
        }

        if let Some(id) = items[0]["id"].as_u64() {
            return Ok(id as u32);
        }
    }

    Err(anyhow!("No game found on Steam for: {}", game_name))
}

async fn verify_url(url: &str) -> bool {
    if let Ok(client) = reqwest::Client::builder().user_agent("OptiTux-GUI").build() {
        if let Ok(resp) = client.head(url).send().await {
            return resp.status().is_success();
        }
    }
    false
}
