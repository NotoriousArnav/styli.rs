use anyhow::{Context, Result};
use std::path::PathBuf;
use tracing::info;

pub async fn download_file(url: &str, output_dir: &PathBuf) -> Result<PathBuf> {
    info!("Downloading from: {}", url);

    let client = reqwest::Client::builder()
        .user_agent("styli-rs/0.1.0")
        .build()
        .context("Failed to build HTTP client")?;

    let response = client
        .get(url)
        .send()
        .await
        .context("Failed to send HTTP request")?;

    if !response.status().is_success() {
        anyhow::bail!(
            "HTTP request failed with status {}: {}",
            response.status(),
            url
        );
    }

    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("image/jpeg")
        .to_string();

    let extension = match content_type.as_str() {
        "image/jpeg" | "image/jpg" => "jpg",
        "image/png" => "png",
        "image/gif" => "gif",
        "image/webp" => "webp",
        "image/bmp" => "bmp",
        _ => "jpg",
    };

    let filename = format!("styli_{}.{}", uuid_simple(), extension);
    let output_path = output_dir.join(&filename);

    let bytes = response
        .bytes()
        .await
        .context("Failed to read response body")?;

    std::fs::write(&output_path, &bytes).context("Failed to write file")?;

    info!("Saved to: {}", output_path.display());
    Ok(output_path)
}

fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!("{:x}{:x}", duration.as_secs(), duration.subsec_nanos())
}
