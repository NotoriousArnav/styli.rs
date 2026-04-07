use anyhow::{Context, Result};
use std::process::Command;
use tracing::info;

pub fn set_feh(wallpaper: &std::path::Path, bgtype: &str) -> Result<()> {
    let wallpaper_str = wallpaper.to_string_lossy();

    let bg_flag = match bgtype {
        "center" => "--bg-center",
        "fill" => "--bg-fill",
        "fit" => "--bg-max",
        "stretch" => "--bg-scale",
        "tile" => "--bg-tile",
        _ => "--bg-fill",
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

pub fn is_feh_available() -> bool {
    Command::new("feh")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
