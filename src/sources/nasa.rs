use crate::download::download_file;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tracing::info;

pub async fn fetch_nasa(output_dir: &Path, api_key: &str) -> Result<PathBuf> {
    let url = format!(
        "https://api.nasa.gov/planetary/apod?api_key={}&thumbs=true",
        api_key
    );

    info!("Fetching from NASA APOD");

    let client = reqwest::Client::builder()
        .user_agent("styli-rs/0.1.0")
        .build()
        .context("Failed to build HTTP client")?;

    let response = client
        .get(&url)
        .send()
        .await
        .context("Failed to fetch NASA APOD")?;

    let json: serde_json::Value = response
        .json()
        .await
        .context("Failed to parse NASA response")?;

    let media_type = json["media_type"].as_str().unwrap_or("image");

    if media_type != "image" {
        anyhow::bail!("Today's APOD is not an image (type: {})", media_type);
    }

    let image_url = if let Some(url) = json["url"].as_str() {
        url.to_string()
    } else if let Some(url) = json["hdurl"].as_str() {
        url.to_string()
    } else {
        anyhow::bail!("No image URL found in NASA response");
    };

    info!("Downloading: {}", image_url);
    download_file(&image_url, output_dir).await
}
