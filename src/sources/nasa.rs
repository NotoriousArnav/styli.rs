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

    if let Some(error) = json.get("error") {
        let msg = error["message"]
            .as_str()
            .unwrap_or("Unknown NASA API error");
        anyhow::bail!("NASA API error: {}", msg);
    }

    let media_type = json["media_type"].as_str().unwrap_or("image");

    if media_type == "video" {
        if let Some(thumb) = json["thumbnail_url"].as_str() {
            info!("APOD is video, using thumbnail: {}", thumb);
            return download_file(thumb, output_dir).await;
        }
        anyhow::bail!("APOD is video with no thumbnail");
    }

    if media_type != "image" {
        anyhow::bail!("APOD is not an image (type: {})", media_type);
    }

    let image_url = json["hdurl"]
        .as_str()
        .or_else(|| json["url"].as_str())
        .map(String::from);

    let url = image_url.context("No image URL in NASA response")?;

    info!("Downloading: {}", url);
    download_file(&url, output_dir).await
}
