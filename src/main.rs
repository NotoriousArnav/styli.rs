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
mod wal;

use sources::picsum::fetch_picsum;
use wallpaper::{Backend, BgType, WallpaperBackend};
use wallpaper::feh::FehBackend;
use wallpaper::nitrogen::NitrogenBackend;
use wallpaper::gnome::GnomeBackend;
use wallpaper::kde::KdeBackend;
use wallpaper::xfce::XfceBackend;
use wallpaper::sway::SwayBackend;
use wallpaper::hyprland::HyprlandBackend;
use wallpaper::awww::AwwwBackend;
use wallpaper::custom::CustomBackend;
use wallpaper::auto;

#[derive(Parser, Debug)]
#[command(name = "styli-rs")]
#[command(author = "Arnav <arnav@styli.sh>")]
#[command(version = "0.1.0")]
#[command(about = "Fast wallpaper switcher with wallust color integration", long_about = None)]
enum Cli {
    #[command(about = "Generate colors from an image (wallust/pywal16 compatible)")]
    Wal(WalArgs),
    
    #[command(about = "Set wallpaper and generate colors")]
    Set(SetArgs),
}

#[derive(Parser, Debug)]
struct WalArgs {
    #[arg(help = "Image file or directory")]
    image: Option<PathBuf>,

    #[arg(short = 'i', long = "image", help = "Image file or directory")]
    image_flag: Option<PathBuf>,

    #[arg(short = 'l', long = "light", help = "Generate light color palette")]
    light: bool,

    #[arg(short = 's', long = "skip-terminal", help = "Skip setting terminal colors")]
    skip_terminal: bool,

    #[arg(short = 'n', long = "skip-wallpaper", help = "Skip wallpaper-related operations")]
    skip_wallpaper: bool,

    #[arg(short = 'q', long = "quiet", help = "Suppress output")]
    quiet: bool,

    #[arg(short = 'p', long = "preview", help = "Preview colors only")]
    preview: bool,

    #[arg(short = 'w', long = "overwrite-cache", help = "Force regeneration, overwrite cache")]
    overwrite_cache: bool,

    #[arg(short = 'b', long = "backend", help = "Color extraction backend")]
    backend: Option<String>,

    #[arg(short = 'c', long = "colorspace", help = "Colorspace algorithm")]
    colorspace: Option<String>,

    #[arg(long = "palette", help = "Palette scheme")]
    palette: Option<String>,

    #[arg(long = "saturation", help = "Saturation level (0-1)")]
    saturation: Option<f32>,

    #[arg(long = "cols16", help = "Use 16 color output (pywal16 compat)")]
    cols16: bool,

    #[arg(short = 'R', long = "restore", help = "Restore previous colorscheme")]
    restore: bool,

    #[arg(short = 'o', long = "other", help = "Run custom script after generation")]
    other: Option<String>,

    #[arg(short = 't', long = "skip-tty", help = "Skip TTY color setting")]
    skip_tty: bool,

    #[arg(short = 'e', long = "skip-reload", help = "Skip reloading desktop environments")]
    skip_reload: bool,
}

#[derive(Parser, Debug)]
struct SetArgs {
    #[arg(short = 's', long, help = "Wallpaper source", default_value = "picsum")]
    source: Source,

    #[arg(short = 'r', long, help = "Wallpaper resolution", default_value = "auto")]
    resolution: Option<String>,

    #[arg(long = "backend", help = "Wallpaper backend", default_value = "auto")]
    backend: Backend,

    #[arg(short = 'm', long = "mode", help = "Background fill mode", default_value = "fill")]
    bgtype: BgType,

    #[arg(long, help = "Skip color generation", alias = "no-colors")]
    no_colors: bool,

    #[arg(long, help = "Color generation options (passed to wal engine)")]
    colors: Option<String>,

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

    #[arg(long = "custom-cmd", help = "Custom wallpaper setter command")]
    custom_cmd: Option<String>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Source {
    Picsum,
    Unsplash,
    Reddit,
    Deviantart,
    Local,
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

fn create_backend(backend: Backend, custom_cmd: Option<&str>) -> Box<dyn WallpaperBackend> {
    match backend {
        Backend::Feh => Box::new(FehBackend::new()),
        Backend::Nitrogen => Box::new(NitrogenBackend::new()),
        Backend::Gnome => Box::new(GnomeBackend::new()),
        Backend::Kde => Box::new(KdeBackend::new()),
        Backend::Xfce => Box::new(XfceBackend::new()),
        Backend::Sway => Box::new(SwayBackend::new()),
        Backend::Hyprland => Box::new(HyprlandBackend::new()),
        Backend::Awww => Box::new(AwwwBackend::new()),
        Backend::Custom => {
            let cmd = custom_cmd.unwrap_or("feh --bg-{bgtype} {wallpaper}");
            Box::new(CustomBackend::new(cmd.to_string()))
        }
        Backend::Auto => {
            let detected = auto::detect();
            create_backend(detected, custom_cmd)
        }
    }
}

fn run_wal(args: WalArgs) -> Result<()> {
    if args.restore {
        info!("Restoring previous colorscheme...");
        let seq_path = dirs::cache_dir()
            .context("Could not find cache directory")?
            .join("wal")
            .join("sequences");
        
        if seq_path.exists() {
            let sequences = std::fs::read_to_string(&seq_path)?;
            if let Ok(pts_dir) = std::fs::read_dir("/dev/pts") {
                for entry in pts_dir.flatten() {
                    let path: std::path::PathBuf = entry.path();
                    if let Ok(mut file) = std::fs::OpenOptions::new().write(true).open(&path) {
                        use std::io::Write;
                        let _ = file.write_all(sequences.as_bytes());
                    }
                }
            }
            info!("Colors restored successfully");
        } else {
            anyhow::bail!("No previous colorscheme found");
        }
        return Ok(());
    }

    let image = args.image.clone().or(args.image_flag.clone())
        .context("No image provided. Use: styli-rs wal <image>")?;

    if !image.exists() {
        anyhow::bail!("Image file not found: {:?}", image);
    }

    let backend = match args.backend.as_deref() {
        Some("kmeans") => wal::Backend::Kmeans,
        Some("wal") => wal::Backend::Wal,
        _ => wal::Backend::FastResize,
    };

    let colorspace = match args.colorspace.as_deref() {
        Some("lab") => wal::Colorspace::Lab,
        Some("lch") => wal::Colorspace::Lch,
        _ => wal::Colorspace::Lab,
    };

    let palette = match args.palette.as_deref() {
        Some("light") => wal::Palette::Light,
        Some("dark16") => wal::Palette::Dark16,
        _ => wal::Palette::Dark,
    };

    let options = wal::WalOptions {
        image,
        backend,
        colorspace,
        palette,
        light: args.light,
        saturation: 0.5,
        skip_terminal: args.skip_terminal,
        skip_wallpaper: args.skip_wallpaper,
        quiet: args.quiet,
        preview: args.preview,
        overwrite_cache: args.overwrite_cache,
    };

    wal::run(options)?;
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

    let cli = Cli::parse();

    match cli {
        Cli::Wal(args) => {
            run_wal(args)?;
        }
        Cli::Set(args) => {
            set_wallpaper(args).await?;
        }
    }

    Ok(())
}

async fn set_wallpaper(args: SetArgs) -> Result<()> {
    info!("styli-rs v0.1.0 starting...");

    let _config = if let Some(config_path) = &args.config {
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
        let backend = create_backend(args.backend, args.custom_cmd.as_deref());
        
        if backend.is_available() {
            info!("Setting wallpaper with {:?}...", args.backend);
            backend
                .set_wallpaper(&wallpaper_path, &args.bgtype)
                .context("Failed to set wallpaper")?;
        } else {
            error!("Backend {:?} is not available", args.backend);
            error!("Try installing the required program or use --backend auto");
            anyhow::bail!("Backend not available");
        }
    }

    if !args.no_colors && !args.preview {
        info!("Generating colors...");
        
        let wal_options = wal::WalOptions {
            image: wallpaper_path.clone(),
            backend: wal::Backend::FastResize,
            colorspace: wal::Colorspace::Lab,
            palette: if args.light { wal::Palette::Light } else { wal::Palette::Dark },
            light: args.light,
            saturation: 0.5,
            skip_terminal: false,
            skip_wallpaper: true,
            quiet: false,
            preview: false,
            overwrite_cache: false,
        };
        
        if let Err(e) = wal::run(wal_options) {
            error!("Failed to generate colors: {}", e);
            error!("Continuing without color generation...");
        }
    }

    if args.preview {
        info!("Previewing colors...");
        let wal_options = wal::WalOptions {
            image: wallpaper_path.clone(),
            backend: wal::Backend::FastResize,
            colorspace: wal::Colorspace::Lab,
            palette: wal::Palette::Dark,
            light: false,
            saturation: 0.5,
            skip_terminal: true,
            skip_wallpaper: true,
            quiet: false,
            preview: true,
            overwrite_cache: false,
        };
        wal::run(wal_options)?;
    }

    info!("Done!");
    Ok(())
}
