#![allow(dead_code)]

use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize, Default)]
pub struct AppConfig {
    #[serde(default)]
    pub general: GeneralConfig,

    #[serde(default)]
    pub wallpaper: WallpaperConfig,

    #[serde(default)]
    pub custom: CustomConfig,

    #[serde(default)]
    pub colors: ColorsConfig,

    #[serde(default)]
    pub reddit: RedditConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GeneralConfig {
    #[serde(default)]
    pub daemon: bool,

    #[serde(default = "default_interval")]
    pub interval: String,
}

fn default_interval() -> String {
    "15m".to_string()
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            daemon: false,
            interval: "15m".to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct WallpaperConfig {
    #[serde(default = "default_source")]
    pub source: String,

    #[serde(default)]
    pub local_dir: Option<String>,

    #[serde(default = "default_resolution")]
    pub resolution: String,

    #[serde(default = "default_backend")]
    pub backend: String,

    #[serde(default = "default_bgtype")]
    pub bgtype: String,
}

fn default_source() -> String {
    "picsum".to_string()
}

fn default_resolution() -> String {
    "auto".to_string()
}

fn default_backend() -> String {
    "feh".to_string()
}

fn default_bgtype() -> String {
    "fill".to_string()
}

impl Default for WallpaperConfig {
    fn default() -> Self {
        Self {
            source: default_source(),
            local_dir: None,
            resolution: default_resolution(),
            backend: default_backend(),
            bgtype: default_bgtype(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CustomConfig {
    #[serde(default)]
    pub command: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ColorsConfig {
    #[serde(default = "default_colors_enabled")]
    pub enabled: bool,

    #[serde(default = "default_cols16")]
    pub cols16: bool,

    #[serde(default = "default_color_backend")]
    pub backend: String,

    #[serde(default = "default_saturate")]
    pub saturate: f32,

    #[serde(default)]
    pub light: bool,

    #[serde(default = "default_compatibility")]
    pub compatibility: bool,
}

fn default_colors_enabled() -> bool {
    true
}

fn default_cols16() -> bool {
    true
}

fn default_color_backend() -> String {
    "kmeans".to_string()
}

fn default_saturate() -> f32 {
    0.5
}

fn default_compatibility() -> bool {
    true
}

impl Default for ColorsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cols16: true,
            backend: default_color_backend(),
            saturate: default_saturate(),
            light: false,
            compatibility: true,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RedditConfig {
    #[serde(default = "default_subreddits")]
    pub subreddits: Vec<String>,

    #[serde(default = "default_sort")]
    pub sort: String,
}

fn default_subreddits() -> Vec<String> {
    vec![
        "wallpapers".to_string(),
        "earthporn".to_string(),
        "nature".to_string(),
    ]
}

fn default_sort() -> String {
    "hot".to_string()
}

impl Default for RedditConfig {
    fn default() -> Self {
        Self {
            subreddits: default_subreddits(),
            sort: default_sort(),
        }
    }
}

pub fn load_config(path: &Path) -> anyhow::Result<AppConfig> {
    let content = fs::read_to_string(path)?;
    let config: AppConfig = toml::from_str(&content)?;
    Ok(config)
}

pub fn load_default_config() -> anyhow::Result<AppConfig> {
    let config_path = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("styli.toml");

    if config_path.exists() {
        load_config(&config_path)
    } else {
        Ok(AppConfig::default())
    }
}
