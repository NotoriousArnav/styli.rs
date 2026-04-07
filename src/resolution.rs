use anyhow::{Context, Result};
use std::process::Command;
use tracing::{debug, info, warn};

pub fn get_resolution() -> Result<String> {
    if let Ok(res) = get_xrandr_resolution() {
        return Ok(res);
    }

    if let Ok(res) = get_wlr_randr_resolution() {
        return Ok(res);
    }

    if let Ok(res) = get_swaymsg_resolution() {
        return Ok(res);
    }

    if let Ok(res) = get_drm_resolution() {
        return Ok(res);
    }

    warn!("Could not detect resolution, falling back to 1920x1080");
    Ok("1920x1080".to_string())
}

fn get_xrandr_resolution() -> Result<String> {
    debug!("Trying xrandr...");
    let output = Command::new("xrandr")
        .arg("--current")
        .output()
        .context("xrandr not found or failed")?;

    if !output.status.success() {
        anyhow::bail!("xrandr failed");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        if line.contains("connected") && !line.contains("disconnected") {
            if let Some(resolution) = parse_xrandr_line(line) {
                info!("Detected resolution (xrandr): {}", resolution);
                return Ok(resolution);
            }
        }
    }

    anyhow::bail!("Could not parse xrandr output")
}

fn parse_xrandr_line(line: &str) -> Option<String> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 3 {
        for part in &parts[1..] {
            if part.contains('x') && part.chars().all(|c| c.is_ascii_digit() || c == 'x') {
                return Some(part.to_string());
            }
        }
    }
    None
}

fn get_wlr_randr_resolution() -> Result<String> {
    debug!("Trying wlr-randr...");
    let output = Command::new("wlr-randr")
        .output()
        .context("wlr-randr not found or failed")?;

    if !output.status.success() {
        anyhow::bail!("wlr-randr failed");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        if line.contains("HDMI")
            || line.contains("DP")
            || line.contains("eDP")
            || line.contains("DisplayPort")
        {
            if let Some(resolution) = parse_wlr_randr_line(line) {
                info!("Detected resolution (wlr-randr): {}", resolution);
                return Ok(resolution);
            }
        }
    }

    anyhow::bail!("Could not parse wlr-randr output")
}

fn parse_wlr_randr_line(line: &str) -> Option<String> {
    let line = line.trim();
    if line.contains("x") {
        let parts: Vec<&str> = line.split_whitespace().collect();
        for part in parts {
            let part = part.trim();
            if part.chars().filter(|c| *c == 'x').count() == 1
                && part.chars().all(|c| c.is_ascii_digit() || c == 'x')
            {
                return Some(part.to_string());
            }
        }
    }
    None
}

fn get_swaymsg_resolution() -> Result<String> {
    debug!("Trying swaymsg...");
    let output = Command::new("swaymsg")
        .args(["-t", "get_outputs"])
        .output()
        .context("swaymsg not found or failed")?;

    if !output.status.success() {
        anyhow::bail!("swaymsg failed");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        if line.contains("\"active\":true") {
            if let Some(res) = extract_sway_resolution(line) {
                info!("Detected resolution (swaymsg): {}", res);
                return Ok(res);
            }
        }
    }

    anyhow::bail!("Could not parse swaymsg output")
}

fn extract_sway_resolution(line: &str) -> Option<String> {
    if let Some(mode_start) = line.find("\"mode\":\"") {
        let mode = &line[mode_start + 8..];
        if let Some(mode_end) = mode.find('"') {
            let mode_str = &mode[..mode_end];
            let parts: Vec<&str> = mode_str.split('x').collect();
            if parts.len() >= 2 {
                return Some(format!("{}x{}", parts[0], parts[1]));
            }
        }
    }
    None
}

fn get_drm_resolution() -> Result<String> {
    debug!("Trying DRM sysfs...");
    let drm_dir = std::path::Path::new("/sys/class/drm");

    for entry in std::fs::read_dir(drm_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.to_string_lossy().contains("card") {
            let status_path = path.join("status");
            if let Ok(status) = std::fs::read_to_string(&status_path) {
                if status.trim() == "connected" {
                    let modes_path = path.join("modes");
                    if let Ok(modes) = std::fs::read_to_string(&modes_path) {
                        if let Some(first_mode) = modes.lines().next() {
                            if first_mode.contains('x') {
                                info!("Detected resolution (DRM): {}", first_mode);
                                return Ok(first_mode.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    anyhow::bail!("Could not read DRM sysfs")
}
