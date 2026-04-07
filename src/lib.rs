pub mod colors;
pub mod config;
pub mod download;
pub mod resolution;
pub mod sources;
pub mod wallpaper;

pub use colors::{generate_colors, preview_colors};
pub use config::{load_config, load_default_config, AppConfig};
pub use download::download_file;
pub use resolution::get_resolution;
pub use sources::picsum::fetch_picsum;
pub use wallpaper::{Backend, BgType, WallpaperBackend};
