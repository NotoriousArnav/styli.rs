use super::{BgType, WallpaperBackend};
use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;
use tracing::info;

pub struct FehBackend;

impl FehBackend {
    pub fn new() -> Self {
        Self
    }
}

impl WallpaperBackend for FehBackend {
    fn name(&self) -> &'static str {
        "feh"
    }

    fn set_wallpaper(&self, wallpaper: &Path, bgtype: &BgType) -> Result<()> {
        let wallpaper_str = wallpaper.to_string_lossy();

        let bg_flag = match bgtype {
            BgType::Center => "--bg-center",
            BgType::Fill => "--bg-fill",
            BgType::Fit => "--bg-max",
            BgType::Stretch => "--bg-scale",
            BgType::Tile => "--bg-tile",
        };

        info!("Running: feh {} {}", bg_flag, wallpaper_str);

        let output = Command::new("feh")
            .arg(bg_flag)
            .arg(&*wallpaper_str)
            .output()
            .context("Failed to execute feh. Is feh installed?")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("feh failed: {}", stderr);
        }

        info!("Wallpaper set successfully with feh");
        Ok(())
    }
}

impl Default for FehBackend {
    fn default() -> Self {
        Self::new()
    }
}
