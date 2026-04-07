use super::{BgType, WallpaperBackend};
use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;
use tracing::info;

pub struct AwwwBackend;

impl AwwwBackend {
    pub fn new() -> Self {
        Self
    }
}

impl WallpaperBackend for AwwwBackend {
    fn name(&self) -> &'static str {
        "awww"
    }

    fn set_wallpaper(&self, wallpaper: &Path, _bgtype: &BgType) -> Result<()> {
        let wallpaper_str = wallpaper.to_string_lossy();

        info!("Setting awww wallpaper: {}", wallpaper_str);

        let output = Command::new("awww")
            .args(["img", &*wallpaper_str])
            .output()
            .context("Failed to execute awww. Is awww running?")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("awww failed: {}", stderr);
        }

        info!("Wallpaper set successfully with awww");
        Ok(())
    }
}

impl Default for AwwwBackend {
    fn default() -> Self {
        Self::new()
    }
}
