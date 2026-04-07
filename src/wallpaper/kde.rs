use super::{BgType, WallpaperBackend};
use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;
use tracing::info;

pub struct KdeBackend;

impl KdeBackend {
    pub fn new() -> Self {
        Self
    }
}

impl WallpaperBackend for KdeBackend {
    fn name(&self) -> &'static str {
        "kde"
    }

    fn set_wallpaper(&self, wallpaper: &Path, _bgtype: &BgType) -> Result<()> {
        let wallpaper_str = wallpaper.to_string_lossy();

        info!("Setting KDE wallpaper: {}", wallpaper_str);

        let plasma_script = format!(
            r#"
            var allPlasmoids = desktops();
            for (var i = 0; i < allPlasmoids.length; i++) {{
                d = allPlasmoids[i];
                d.wallpaperPlugin = "org.kde.image";
                d.currentConfigGroup = Array("Wallpaper", "org.kde.image", "General");
                d.writeConfig("Image", "file://{}");
            }}
            "#,
            wallpaper_str
        );

        let output = Command::new("qdbus")
            .args([
                "org.kde.plasmashell",
                "/PlasmaShell",
                "org.kde.PlasmaShell.evaluateScript",
                &plasma_script,
            ])
            .output()
            .context("Failed to execute qdbus. Is KDE/Plasma running?")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("qdbus failed: {}", stderr);
        }

        info!("Wallpaper set successfully with KDE");
        Ok(())
    }
}

impl Default for KdeBackend {
    fn default() -> Self {
        Self::new()
    }
}
