use super::{BgType, WallpaperBackend};
use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;
use tracing::info;

pub struct SwwwBackend;

impl SwwwBackend {
    pub fn new() -> Self {
        Self
    }
}

impl WallpaperBackend for SwwwBackend {
    fn name(&self) -> &'static str {
        "swww"
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
            "Setting SWWW wallpaper: {} (filter: {})",
            wallpaper_str, filter
        );

        let output = Command::new("swww")
            .args([
                "img",
                &*wallpaper_str,
                "--filter",
                filter,
                "--transition-type",
                "fade",
            ])
            .output()
            .context("Failed to execute swww. Is swww running?")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("swww failed: {}", stderr);
        }

        info!("Wallpaper set successfully with SWWW");
        Ok(())
    }
}

impl Default for SwwwBackend {
    fn default() -> Self {
        Self::new()
    }
}
