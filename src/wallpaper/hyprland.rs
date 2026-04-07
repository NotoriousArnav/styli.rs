use super::{BgType, WallpaperBackend};
use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;
use tracing::info;

pub struct HyprlandBackend;

impl HyprlandBackend {
    pub fn new() -> Self {
        Self
    }
}

impl WallpaperBackend for HyprlandBackend {
    fn name(&self) -> &'static str {
        "hyprland"
    }

    fn set_wallpaper(&self, wallpaper: &Path, _bgtype: &BgType) -> Result<()> {
        let wallpaper_str = wallpaper.to_string_lossy();

        info!("Setting Hyprland wallpaper: {}", wallpaper_str);

        let output = Command::new("hyprctl")
            .args(["hyprpaper", "preload", &*wallpaper_str])
            .output()
            .context("Failed to execute hyprctl. Is Hyprland running?")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("hyprctl preload failed: {}", stderr);
        }

        Command::new("hyprctl")
            .args(["hyprpaper", "wallpaper", ",", &*wallpaper_str])
            .output()
            .ok();

        info!("Wallpaper set successfully with Hyprland");
        Ok(())
    }
}

impl Default for HyprlandBackend {
    fn default() -> Self {
        Self::new()
    }
}
