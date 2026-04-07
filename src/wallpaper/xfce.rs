use super::{BgType, WallpaperBackend};
use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;
use tracing::info;

pub struct XfceBackend;

impl XfceBackend {
    pub fn new() -> Self {
        Self
    }
}

impl WallpaperBackend for XfceBackend {
    fn name(&self) -> &'static str {
        "xfce"
    }

    fn set_wallpaper(&self, wallpaper: &Path, bgtype: &BgType) -> Result<()> {
        let wallpaper_str = wallpaper.to_string_lossy();

        let mode = match bgtype {
            BgType::Center => 1,
            BgType::Fill => 5,
            BgType::Fit => 3,
            BgType::Stretch => 2,
            BgType::Tile => 0,
        };

        info!("Setting XFCE wallpaper: {} (mode: {})", wallpaper_str, mode);

        Command::new("xfconf-query")
            .args([
                "-c",
                "xfce4-desktop",
                "-p",
                "/backdrop/screen0/monitor0/workspace0/last-image",
                "-s",
                &*wallpaper_str,
            ])
            .output()
            .context("Failed to execute xfconf-query. Is XFCE running?")?;

        Command::new("xfconf-query")
            .args([
                "-c",
                "xfce4-desktop",
                "-p",
                "/backdrop/screen0/monitor0/workspace0/image-style",
                "-s",
                &mode.to_string(),
            ])
            .output()
            .ok();

        info!("Wallpaper set successfully with XFCE");
        Ok(())
    }
}

impl Default for XfceBackend {
    fn default() -> Self {
        Self::new()
    }
}
