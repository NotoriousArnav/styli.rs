use anyhow::{Context, Result};
use std::path::Path;
use tracing::info;

#[allow(dead_code)]
pub async fn generate_colors(wallpaper: &Path, light: bool) -> Result<()> {
    info!("Generating colors with wallust...");

    let wallpaper_str = wallpaper.to_string_lossy();

    let mut cmd = std::process::Command::new("wallust");
    cmd.arg("run").arg(&*wallpaper_str);

    if light {
        cmd.arg("--light");
    }

    info!("Running: {:?}", cmd);

    let output = cmd.output().context("Failed to run wallust")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        tracing::error!("wallust stdout: {}", stdout);
        tracing::error!("wallust stderr: {}", stderr);
        anyhow::bail!("wallust failed");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    info!("wallust output: {}", stdout);
    info!("Colors generated successfully");
    Ok(())
}

#[allow(dead_code)]
pub async fn preview_colors(wallpaper: &Path) -> Result<()> {
    info!("Previewing colors with wallust...");

    let wallpaper_str = wallpaper.to_string_lossy();

    let output = std::process::Command::new("wallust")
        .arg("run")
        .arg(&*wallpaper_str)
        .arg("--preview")
        .output()
        .context("Failed to run wallust --preview")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("wallust --preview failed: {}", stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{}", stdout);
    Ok(())
}
