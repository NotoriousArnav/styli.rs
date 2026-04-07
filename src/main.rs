use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use tracing::{error, info};

mod colors;
mod config;
mod download;
mod resolution;
mod sources;
mod wallpaper;

use sources::picsum::fetch_picsum;
use wallpaper::feh::set_feh;

#[derive(Parser, Debug)]
#[command(name = "styli-rs")]
#[command(author = "Arnav <arnav@styli.sh>")]
#[command(version = "0.1.0")]
#[command(about = "Fast wallpaper switcher with wallust color integration", long_about = None)]
struct Args {
    #[arg(short = 's', long, help = "Wallpaper source", default_value = "picsum")]
    source: Source,

    #[arg(short = 'r', long, help = "Wallpaper resolution", default_value = "auto")]
    resolution: Option<String>,

    #[arg(long = "backend", help = "Wallpaper backend", default_value = "feh")]
    backend: Backend,

    #[arg(short = 'm', long = "mode", help = "Background fill mode", default_value = "fill")]
    bgtype: BgType,

    #[arg(long, help = "Skip color generation")]
    no_colors: bool,

    #[arg(long, help = "Generate light color palette")]
    light: bool,

    #[arg(long, help = "Reload colors from last wallpaper")]
    reload: bool,

    #[arg(long, help = "Preview colors without setting wallpaper")]
    preview: bool,

    #[arg(short = 'c', long = "config", help = "Config file path")]
    config: Option<PathBuf>,

    #[arg(short = 'o', long = "output", help = "Output directory for wallpaper")]
    output: Option<PathBuf>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Source {
    Picsum,
    Unsplash,
    Reddit,
    Deviantart,
    Local,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Backend {
    Feh,
    Nitrogen,
    Gnome,
    Kde,
    Xfce,
    Sway,
    Hyprland,
    Swww,
    Custom,
    Auto,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum BgType {
    Center,
    Fill,
    Fit,
    Stretch,
    Tile,
}

impl BgType {
    fn as_str(&self) -> &'static str {
        match self {
            BgType::Center => "center",
            BgType::Fill => "fill",
            BgType::Fit => "fit",
            BgType::Stretch => "stretch",
            BgType::Tile => "tile",
        }
    }
}

fn get_cache_dir() -> PathBuf {
    dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from(".cache"))
        .join("styli-rs")
}

fn get_wallpaper_output_dir() -> PathBuf {
    dirs::picture_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("wallpapers")
}

fn get_last_wallpaper() -> Option<PathBuf> {
    let cache_file = get_cache_dir().join("last_wallpaper");
    std::fs::read_to_string(cache_file).ok().map(PathBuf::from)
}

fn save_last_wallpaper(path: &PathBuf) -> Result<()> {
    let cache_dir = get_cache_dir();
    std::fs::create_dir_all(&cache_dir)?;
    std::fs::write(cache_dir.join("last_wallpaper"), path.to_string_lossy().as_ref())?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let args = Args::parse();
    info!("styli-rs v0.1.0 starting...");

    let config = if let Some(config_path) = &args.config {
        config::load_config(config_path)?
    } else {
        config::load_default_config().unwrap_or_default()
    };

    let resolution = if let Some(ref res) = args.resolution {
        if res != "auto" {
            Some(res.clone())
        } else {
            None
        }
    } else {
        None
    };

    let resolution_str = match resolution {
        Some(ref r) => r.clone(),
        None => {
            info!("Detecting monitor resolution...");
            resolution::get_resolution().unwrap_or_else(|e| {
                error!("Failed to detect resolution: {}", e);
                "1920x1080".to_string()
            })
        }
    };

    let wallpaper_path = if args.reload {
        info!("Reloading colors from last wallpaper...");
        match get_last_wallpaper() {
            Some(path) => {
                if !path.exists() {
                    anyhow::bail!("Last wallpaper not found: {}", path.display());
                }
                path
            }
            None => anyhow::bail!("No previous wallpaper found. Run without --reload first."),
        }
    } else {
        let output_dir = args.output.unwrap_or_else(get_wallpaper_output_dir);
        std::fs::create_dir_all(&output_dir)?;

        info!("Fetching wallpaper from {:?}...", args.source);
        match args.source {
            Source::Picsum => {
                fetch_picsum(&output_dir, &resolution_str).await?
            }
            Source::Unsplash => {
                anyhow::bail!("Unsplash source not implemented yet");
            }
            Source::Reddit => {
                anyhow::bail!("Reddit source not implemented yet");
            }
            Source::Deviantart => {
                anyhow::bail!("DeviantArt source not implemented yet");
            }
            Source::Local => {
                anyhow::bail!("Local source not implemented yet");
            }
        }
    };

    info!("Wallpaper saved to: {}", wallpaper_path.display());
    save_last_wallpaper(&wallpaper_path)?;

    if args.preview {
        info!("Preview mode - skipping wallpaper set");
    } else {
        info!("Setting wallpaper with {:?}...", args.backend);
        match args.backend {
            Backend::Feh => {
                set_feh(&wallpaper_path, args.bgtype.as_str())
                    .context("Failed to set wallpaper with feh")?;
            }
            Backend::Nitrogen => {
                anyhow::bail!("Nitrogen backend not implemented yet");
            }
            Backend::Gnome => {
                anyhow::bail!("GNOME backend not implemented yet");
            }
            Backend::Kde => {
                anyhow::bail!("KDE backend not implemented yet");
            }
            Backend::Xfce => {
                anyhow::bail!("XFCE backend not implemented yet");
            }
            Backend::Sway => {
                anyhow::bail!("Sway backend not implemented yet");
            }
            Backend::Hyprland => {
                anyhow::bail!("Hyprland backend not implemented yet");
            }
            Backend::Swww => {
                anyhow::bail!("SWWW backend not implemented yet");
            }
            Backend::Custom => {
                anyhow::bail!("Custom backend not implemented yet");
            }
            Backend::Auto => {
                set_feh(&wallpaper_path, args.bgtype.as_str())
                    .context("Failed to set wallpaper with feh")?;
            }
        }
    }

    if !args.no_colors && !args.preview {
        info!("Generating colors with wallust...");
        if let Err(e) = colors::generate_colors(&wallpaper_path, args.light).await {
            error!("Failed to generate colors: {}", e);
            error!("Continuing without color generation...");
        }
    }

    if args.preview {
        info!("Previewing colors...");
        colors::preview_colors(&wallpaper_path).await?;
    }

    info!("Done!");
    Ok(())
}
