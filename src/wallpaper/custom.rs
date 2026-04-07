use super::{BgType, WallpaperBackend};
use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;
use tracing::info;

pub struct CustomBackend {
    pub command: String,
}

impl CustomBackend {
    pub fn new(command: String) -> Self {
        Self { command }
    }
}

impl WallpaperBackend for CustomBackend {
    fn name(&self) -> &'static str {
        "custom"
    }

    fn is_available(&self) -> bool {
        true
    }

    fn set_wallpaper(&self, wallpaper: &Path, bgtype: &BgType) -> Result<()> {
        let wallpaper_str = wallpaper.to_string_lossy();
        let resolution =
            crate::resolution::get_resolution().unwrap_or_else(|_| "1920x1080".to_string());

        let expanded = self
            .command
            .replace("{wallpaper}", &wallpaper_str)
            .replace("{bgtype}", bgtype.as_str())
            .replace("{resolution}", &resolution);

        info!("Running custom command: {}", expanded);

        let output = Command::new("sh")
            .arg("-c")
            .arg(&expanded)
            .output()
            .context("Custom backend command failed")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Custom backend failed: {}", stderr);
        }

        info!("Wallpaper set successfully with custom backend");
        Ok(())
    }
}
