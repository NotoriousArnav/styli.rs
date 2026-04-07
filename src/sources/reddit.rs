use crate::download::download_file;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tracing::info;

pub async fn fetch_reddit(
    output_dir: &Path,
    resolution: &str,
    subreddits: &[String],
    sort: &str,
) -> Result<PathBuf> {
    let (_width, _height) = parse_resolution(resolution);

    let default_sub = "wallpapers".to_string();
    let subreddit = subreddits.first().unwrap_or(&default_sub);
    let sort_param = match sort {
        "top" => "top",
        "rising" => "rising",
        "hot" => "hot",
        _ => "hot",
    };

    let url = format!(
        "https://www.reddit.com/r/{}/{}.json?limit=100",
        subreddit, sort_param
    );

    info!("Fetching from Reddit: r/{}", subreddit);

    let client = reqwest::Client::builder()
        .user_agent("styli-rs/0.1.0")
        .build()
        .context("Failed to build HTTP client")?;

    let response = client
        .get(&url)
        .header("Accept", "application/json")
        .send()
        .await
        .context("Failed to fetch Reddit posts")?;

    let json: serde_json::Value = response
        .json()
        .await
        .context("Failed to parse Reddit response")?;

    let posts = json["data"]["children"]
        .as_array()
        .context("Failed to parse Reddit posts")?;

    for post in posts {
        let post_data = &post["data"];
        
        let is_image = post_data["post_hint"].as_str() == Some("image")
            || post_data["url"]
                .as_str()
                .map(|u| u.ends_with(".jpg") || u.ends_with(".png") || u.ends_with(".webp"))
                .unwrap_or(false);

        if !is_image {
            continue;
        }

        let image_url = post_data["url"].as_str().unwrap_or("");
        
        if image_url.contains("preview.reddit.com") || image_url.contains("i.redd.it") {
            if let Some(media) = post_data["preview"]["images"].as_array() {
                if let Some(source) = media[0]["source"].as_object() {
                    let url = source["url"].as_str().unwrap_or("");
                    let image_url = url.replace("&amp;", "&");
                    
                    info!("Downloading: {}", image_url);
                    match download_file(&image_url, output_dir).await {
                        Ok(path) => return Ok(path),
                        Err(e) => {
                            info!("Failed to download: {}", e);
                            continue;
                        }
                    }
                }
            }
        }

        if image_url.ends_with(".jpg") || image_url.ends_with(".png") || image_url.ends_with(".webp") {
            match download_file(image_url, output_dir).await {
                Ok(path) => return Ok(path),
                Err(e) => {
                    info!("Failed to download: {}", e);
                    continue;
                }
            }
        }
    }

    anyhow::bail!("No valid images found on Reddit")
}

pub async fn fetch_local(output_dir: &Path, folder: &Path) -> Result<PathBuf> {
    info!("Fetching from local folder: {:?}", folder);

    let entries = std::fs::read_dir(folder).context("Failed to read local folder")?;

    let mut images: Vec<_> = entries
        .filter_map(|e| e.ok())
        .filter(|e| {
            let path = e.path();
            if let Some(ext) = path.extension() {
                let ext = ext.to_string_lossy().to_lowercase();
                matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp")
            } else {
                false
            }
        })
        .collect();

    if images.is_empty() {
        anyhow::bail!("No images found in local folder");
    }

    use std::time::SystemTime;
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    let idx = (now % images.len() as u64) as usize;

    let selected = &images[idx];
    let filename = format!("styli_{}.local", now);
    let output_path = output_dir.join(&filename);

    std::fs::copy(selected.path(), &output_path).context("Failed to copy image")?;

    info!("Copied: {} -> {}", selected.path().display(), output_path.display());
    Ok(output_path)
}

fn parse_resolution(resolution: &str) -> (u32, u32) {
    let parts: Vec<&str> = resolution.split('x').collect();
    if parts.len() >= 2 {
        let width = parts[0].parse().unwrap_or(1920);
        let height = parts[1].parse().unwrap_or(1080);
        (width, height)
    } else {
        (1920, 1080)
    }
}
