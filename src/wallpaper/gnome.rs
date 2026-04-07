use super::{BgType, WallpaperBackend};
use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;
use tracing::info;

pub struct GnomeBackend;

impl GnomeBackend {
    pub fn new() -> Self {
        Self
    }
}

impl WallpaperBackend for GnomeBackend {
    fn name(&self) -> &'static str {
        "gnome"
    }

    fn set_wallpaper(&self, wallpaper: &Path, _bgtype: &BgType) -> Result<()> {
        let wallpaper_str = wallpaper.to_string_lossy();

        info!("Setting GNOME wallpaper: {}", wallpaper_str);

        let output = Command::new("gsettings")
            .args([
                "set",
                "org.gnome.desktop.background",
                "picture-uri",
                &format!("file://{}", wallpaper_str),
            ])
            .output()
            .context("Failed to execute gsettings. Is gsettings installed?")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("gsettings failed: {}", stderr);
        }

        Command::new("gsettings")
            .args([
                "set",
                "org.gnome.desktop.background",
                "picture-uri-dark",
                &format!("file://{}", wallpaper_str),
            ])
            .output()
            .ok();

        info!("Wallpaper set successfully with GNOME");
        Ok(())
    }
}

impl Default for GnomeBackend {
    fn default() -> Self {
        Self::new()
    }
}
