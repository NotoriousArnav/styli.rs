use super::{BgType, WallpaperBackend};
use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;
use tracing::info;

pub struct NitrogenBackend;

impl NitrogenBackend {
    pub fn new() -> Self {
        Self
    }
}

impl WallpaperBackend for NitrogenBackend {
    fn name(&self) -> &'static str {
        "nitrogen"
    }

    fn set_wallpaper(&self, wallpaper: &Path, _bgtype: &BgType) -> Result<()> {
        let wallpaper_str = wallpaper.to_string_lossy();

        info!("Running: nitrogen --set-zoom-fill {}", wallpaper_str);

        let output = Command::new("nitrogen")
            .args(["--set-zoom-fill", &*wallpaper_str])
            .output()
            .context("Failed to execute nitrogen. Is nitrogen installed?")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("nitrogen failed: {}", stderr);
        }

        info!("Wallpaper set successfully with nitrogen");
        Ok(())
    }
}

impl Default for NitrogenBackend {
    fn default() -> Self {
        Self::new()
    }
}
