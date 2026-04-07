use anyhow::Result;
use clap::ValueEnum;
use std::path::Path;
use std::process::Command;

pub trait WallpaperBackend: Send + Sync {
    fn name(&self) -> &'static str;

    fn set_wallpaper(&self, wallpaper: &Path, bgtype: &BgType) -> Result<()>;

    fn is_available(&self) -> bool {
        true
    }
}

pub fn check_command(cmd: &str) -> bool {
    Command::new(cmd)
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Backend {
    Feh,
    Nitrogen,
    Gnome,
    Kde,
    Xfce,
    Sway,
    Hyprland,
    Awww,
    Custom,
    Auto,
}

impl Backend {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "feh" => Backend::Feh,
            "nitrogen" => Backend::Nitrogen,
            "gnome" => Backend::Gnome,
            "kde" => Backend::Kde,
            "xfce" => Backend::Xfce,
            "sway" => Backend::Sway,
            "hyprland" => Backend::Hyprland,
            "awww" => Backend::Awww,
            "custom" => Backend::Custom,
            "auto" => Backend::Auto,
            _ => Backend::Feh,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum BgType {
    Center,
    Fill,
    Fit,
    Stretch,
    Tile,
}

impl BgType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "center" => BgType::Center,
            "fill" => BgType::Fill,
            "fit" => BgType::Fit,
            "stretch" => BgType::Stretch,
            "tile" => BgType::Tile,
            _ => BgType::Fill,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            BgType::Center => "center",
            BgType::Fill => "fill",
            BgType::Fit => "fit",
            BgType::Stretch => "stretch",
            BgType::Tile => "tile",
        }
    }
}

pub mod auto;
pub mod awww;
pub mod custom;
pub mod feh;
pub mod gnome;
pub mod hyprland;
pub mod kde;
pub mod nitrogen;
pub mod sway;
pub mod xfce;
