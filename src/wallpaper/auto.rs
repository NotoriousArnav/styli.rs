use super::Backend;
use tracing::info;

pub fn detect() -> Backend {
    if std::process::Command::new("swaymsg")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        info!("Auto-detected wallpaper backend: sway");
        return Backend::Sway;
    }

    if std::process::Command::new("hyprctl")
        .arg("version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        info!("Auto-detected wallpaper backend: hyprland");
        return Backend::Hyprland;
    }

    if std::process::Command::new("aww")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        info!("Auto-detected wallpaper backend: aww");
        return Backend::Aww;
    }

    if std::process::Command::new("gsettings")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        info!("Auto-detected wallpaper backend: gnome");
        return Backend::Gnome;
    }

    if std::process::Command::new("qdbus")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        info!("Auto-detected wallpaper backend: kde");
        return Backend::Kde;
    }

    if std::process::Command::new("xfconf-query")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        info!("Auto-detected wallpaper backend: xfce");
        return Backend::Xfce;
    }

    if std::process::Command::new("nitrogen")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        info!("Auto-detected wallpaper backend: nitrogen");
        return Backend::Nitrogen;
    }

    if std::process::Command::new("feh")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        info!("Auto-detected wallpaper backend: feh");
        return Backend::Feh;
    }

    info!("No wallpaper backend detected, defaulting to feh");
    Backend::Feh
}
