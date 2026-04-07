use super::{BgType, WallpaperBackend};
use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;
use tracing::info;

pub struct AwwBackend;

impl AwwBackend {
    pub fn new() -> Self {
        Self
    }
}

impl WallpaperBackend for AwwBackend {
    fn name(&self) -> &'static str {
        "aww"
    }

    fn set_wallpaper(&self, wallpaper: &Path, bgtype: &BgType) -> Result<()> {
        let wallpaper_str = wallpaper.to_string_lossy();

        let filter = match bgtype {
            BgType::Center => "center",
            BgType::Fill => "crop",
            BgType::Fit => "fit",
            BgType::Stretch => "stretch",
            BgType::Tile => "tile",
        };

        info!(
            "Setting aww wallpaper: {} (filter: {})",
            wallpaper_str, filter
        );

        let output = Command::new("aww")
            .args([
                "img",
                &*wallpaper_str,
                "--filter",
                filter,
                "--transition-type",
                "fade",
            ])
            .output()
            .context("Failed to execute aww. Is aww running?")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("aww failed: {}", stderr);
        }

        info!("Wallpaper set successfully with aww");
        Ok(())
    }
}

impl Default for AwwBackend {
    fn default() -> Self {
        Self::new()
    }
}
