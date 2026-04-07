use crate::download::download_file;
use anyhow::Result;
use std::path::{Path, PathBuf};
use tracing::info;

pub async fn fetch_picsum(output_dir: &Path, resolution: &str) -> Result<PathBuf> {
    let (width, height) = parse_resolution(resolution);

    let url = format!("https://picsum.photos/{}/{}", width, height);

    info!("Fetching from Picsum: {}x{}", width, height);
    download_file(&url, output_dir).await
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
