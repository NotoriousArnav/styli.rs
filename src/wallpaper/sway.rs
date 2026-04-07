use super::{BgType, WallpaperBackend};
use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;
use tracing::info;

pub struct SwayBackend;

impl SwayBackend {
    pub fn new() -> Self {
        Self
    }
}

impl WallpaperBackend for SwayBackend {
    fn name(&self) -> &'static str {
        "sway"
    }

    fn set_wallpaper(&self, wallpaper: &Path, bgtype: &BgType) -> Result<()> {
        let wallpaper_str = wallpaper.to_string_lossy();

        let mode = match bgtype {
            BgType::Center => "center",
            BgType::Fill => "fill",
            BgType::Fit => "fit",
            BgType::Stretch => "stretch",
            BgType::Tile => "tile",
        };

        info!("Setting Sway wallpaper: {} (mode: {})", wallpaper_str, mode);

        let output = Command::new("swaymsg")
            .args(["output", "*", "bg", &*wallpaper_str, mode])
            .output()
            .context("Failed to execute swaymsg. Is Sway running?")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("swaymsg failed: {}", stderr);
        }

        info!("Wallpaper set successfully with Sway");
        Ok(())
    }
}

impl Default for SwayBackend {
    fn default() -> Self {
        Self::new()
    }
}
