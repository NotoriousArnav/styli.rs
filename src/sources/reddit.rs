use crate::download::download_file;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tracing::info;

pub async fn fetch_reddit(
    output_dir: &Path,
    _resolution: &str,
    subreddits: &[String],
    sort: &str,
) -> Result<PathBuf> {
    let default_sub = "wallpapers".to_string();
    let subreddit = subreddits.first().unwrap_or(&default_sub);
    let sort_param = match sort {
        "top" => "top",
        "rising" => "rising",
        "hot" => "hot",
        _ => "top",
    };

    let url = format!("https://www.reddit.com/r/{}/{}.json", subreddit, sort_param);

    info!("Fetching from Reddit: r/{}", subreddit);

    let output = std::process::Command::new("curl")
        .args([
            "-s", "-L",
            "-A", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/146.0.0.0 Safari/537.36",
            "-H", "Accept: application/json",
            &url
        ])
        .output()
        .context("Failed to run curl")?;

    if !output.status.success() {
        anyhow::bail!("curl failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    let json: serde_json::Value =
        serde_json::from_slice(&output.stdout).context("Failed to parse Reddit response")?;

    let posts = json["data"]["children"]
        .as_array()
        .context("Failed to parse Reddit posts")?;

    info!("Found {} posts", posts.len());

    let mut valid_urls: Vec<String> = Vec::new();

    for post in posts {
        let post_data = &post["data"];

        if let Some(url) = post_data["url_overridden_by_dest"].as_str() {
            if !url.is_empty() {
                valid_urls.push(url.to_string());
            }
        }
    }

    if valid_urls.is_empty() {
        anyhow::bail!("No valid images found on Reddit");
    }

    use std::time::SystemTime;
    let seed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as usize;
    let idx = seed % valid_urls.len();
    let url = &valid_urls[idx];

    info!("Downloading random post #{}: {}", idx, url);
    download_file(url, output_dir).await
}

pub async fn fetch_local(output_dir: &Path, folder: &Path) -> Result<PathBuf> {
    info!("Fetching from local folder: {:?}", folder);

    let entries = std::fs::read_dir(folder).context("Failed to read local folder")?;

    let images: Vec<_> = entries
        .filter_map(|e| e.ok())
        .filter(|e| {
            let path = e.path();
            if let Some(ext) = path.extension() {
                let ext = ext.to_string_lossy().to_lowercase();
                matches!(
                    ext.as_str(),
                    "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp"
                )
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

    info!(
        "Copied: {} -> {}",
        selected.path().display(),
        output_path.display()
    );
    Ok(output_path)
}
