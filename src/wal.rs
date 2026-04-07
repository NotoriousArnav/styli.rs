use anyhow::{Context, Result};
use image::GenericImageView;
use kmeans_colors::{get_kmeans, Kmeans, Sort};
use palette::{cast::from_component_slice, IntoColor, Lab, Srgb};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use tracing::info;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    FastResize,
    Kmeans,
    Wal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Colorspace {
    Salience,
    Lab,
    Lch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Palette {
    Dark,
    Light,
    Dark16,
}

#[derive(Debug)]
pub struct WalOptions {
    pub image: PathBuf,
    pub backend: Backend,
    pub colorspace: Colorspace,
    pub palette: Palette,
    pub light: bool,
    pub saturation: f32,
    pub skip_terminal: bool,
    pub skip_wallpaper: bool,
    pub quiet: bool,
    pub preview: bool,
    pub overwrite_cache: bool,
}

impl Default for WalOptions {
    fn default() -> Self {
        Self {
            image: PathBuf::new(),
            backend: Backend::FastResize,
            colorspace: Colorspace::Lab,
            palette: Palette::Dark,
            light: false,
            saturation: 0.5,
            skip_terminal: false,
            skip_wallpaper: false,
            quiet: false,
            preview: false,
            overwrite_cache: false,
        }
    }
}

pub fn run(options: WalOptions) -> Result<HashMap<String, String>> {
    let img = image::open(&options.image).context("Failed to open image")?;

    if !options.quiet {
        info!("Extracting colors from: {:?}", options.image);
    }

    let colors = extract_colors(&img)?;

    let mut scheme = generate_palette(&colors, options.light);

    if options.preview {
        print_palette(&scheme);
        return Ok(scheme);
    }

    let cache_dir = get_cache_dir()?;
    fs::create_dir_all(&cache_dir).context("Failed to create cache directory")?;

    write_colors_file(&cache_dir, &scheme)?;
    write_sequences(&cache_dir, &scheme)?;

    if !options.skip_terminal {
        apply_terminal_colors(&scheme)?;
    }

    if !options.quiet {
        info!("Colors generated successfully");
    }

    Ok(scheme)
}

fn extract_colors(img: &image::DynamicImage) -> Result<Vec<Srgb<f32>>> {
    let (width, height) = img.dimensions();
    let max_dim: u32 = 512;

    let resized = if width > max_dim || height > max_dim {
        let ratio = max_dim as f32 / width.max(height) as f32;
        let new_width = (width as f32 * ratio) as u32;
        let new_height = (height as f32 * ratio) as u32;
        img.resize(new_width, new_height, image::imageops::FilterType::Triangle)
    } else {
        img.clone()
    };

    let rgb = resized.to_rgb8();
    let pixels: Vec<u8> = rgb.into_raw();

    let colors: Vec<Lab> = from_component_slice::<Srgb<u8>>(&pixels)
        .iter()
        .map(|x| (*x).into_linear().into_color())
        .collect();

    let k = 6;
    let max_iter = 20;
    let converge = 0.001;
    let seed = 42;
    let verbose = false;

    let mut result = Kmeans::new();
    for i in 0..3 {
        let run_result = get_kmeans(k, max_iter, converge, verbose, &colors, seed + i as u64);
        if run_result.score < result.score || i == 0 {
            result = run_result;
        }
    }

    let mut sorted = Lab::sort_indexed_colors(&result.centroids, &result.indices);
    sorted.sort_unstable_by(|a, b| b.percentage.partial_cmp(&a.percentage).unwrap());

    let mut extracted_colors: Vec<Srgb<f32>> = Vec::with_capacity(6);
    for sd in sorted.iter().take(6) {
        let lab: Lab = sd.centroid;
        let rgb: Srgb<f32> = lab.into_color();
        extracted_colors.push(rgb);
    }

    while extracted_colors.len() < 6 {
        extracted_colors.push(Srgb::new(0.0, 0.0, 0.0));
    }

    Ok(extracted_colors)
}

fn generate_palette(colors: &[Srgb<f32>], light: bool) -> HashMap<String, String> {
    let mut scheme: HashMap<String, String> = HashMap::new();

    if light {
        generate_light_palette(colors, &mut scheme);
    } else {
        generate_dark_palette(colors, &mut scheme);
    }

    scheme
}

fn generate_dark_palette(colors: &[Srgb<f32>], scheme: &mut HashMap<String, String>) {
    let ee = Srgb::new(238.0 / 255.0, 238.0 / 255.0, 238.0 / 255.0);

    let lightest = colors.last().copied().unwrap_or(ee);
    let darkest = colors.first().copied().unwrap_or(Srgb::new(0.1, 0.1, 0.1));

    let fg = lighten(lightest, 0.65);

    let (col0, bg) = getbg_dark(darkest);

    let col7 = blend(ee, lightest);
    let col8 = darken(col7, 0.30);
    let col15 = blend(ee, lightest);

    scheme.insert("background".to_string(), rgb_to_hex(bg));
    scheme.insert("foreground".to_string(), rgb_to_hex(fg));

    scheme.insert("color0".to_string(), rgb_to_hex(col0));
    scheme.insert("color1".to_string(), rgb_to_hex(colors[5]));
    scheme.insert("color2".to_string(), rgb_to_hex(colors[4]));
    scheme.insert("color3".to_string(), rgb_to_hex(colors[3]));
    scheme.insert("color4".to_string(), rgb_to_hex(colors[2]));
    scheme.insert("color5".to_string(), rgb_to_hex(colors[1]));
    scheme.insert("color6".to_string(), rgb_to_hex(colors[0]));
    scheme.insert("color7".to_string(), rgb_to_hex(col7));

    scheme.insert("color8".to_string(), rgb_to_hex(col8));
    scheme.insert("color9".to_string(), rgb_to_hex(colors[5]));
    scheme.insert("color10".to_string(), rgb_to_hex(colors[4]));
    scheme.insert("color11".to_string(), rgb_to_hex(colors[3]));
    scheme.insert("color12".to_string(), rgb_to_hex(colors[2]));
    scheme.insert("color13".to_string(), rgb_to_hex(colors[1]));
    scheme.insert("color14".to_string(), rgb_to_hex(colors[0]));
    scheme.insert("color15".to_string(), rgb_to_hex(col15));
}

fn generate_light_palette(colors: &[Srgb<f32>], scheme: &mut HashMap<String, String>) {
    let lightest = colors.last().copied().unwrap();
    let darkest = colors.first().copied().unwrap();

    let (color0, bg) = getbg_light(lightest);
    let fg = darken_fixed(darkest, 0.55);

    let col7 = darken_fixed(darkest, 0.55);
    let col15 = darken_fixed(darkest, 0.85);
    let col8 = darken(lightest, 0.3);

    let color1 = darken_fixed(colors[5], 0.1);

    scheme.insert("background".to_string(), rgb_to_hex(bg));
    scheme.insert("foreground".to_string(), rgb_to_hex(fg));

    scheme.insert("color0".to_string(), rgb_to_hex(color0));
    scheme.insert("color1".to_string(), rgb_to_hex(color1));
    scheme.insert("color2".to_string(), rgb_to_hex(colors[4]));
    scheme.insert("color3".to_string(), rgb_to_hex(colors[3]));
    scheme.insert("color4".to_string(), rgb_to_hex(colors[2]));
    scheme.insert("color5".to_string(), rgb_to_hex(colors[1]));
    scheme.insert("color6".to_string(), rgb_to_hex(colors[0]));
    scheme.insert("color7".to_string(), rgb_to_hex(col7));

    scheme.insert("color8".to_string(), rgb_to_hex(col8));
    scheme.insert("color9".to_string(), rgb_to_hex(color1));
    scheme.insert("color10".to_string(), rgb_to_hex(colors[4]));
    scheme.insert("color11".to_string(), rgb_to_hex(colors[3]));
    scheme.insert("color12".to_string(), rgb_to_hex(colors[2]));
    scheme.insert("color13".to_string(), rgb_to_hex(colors[1]));
    scheme.insert("color14".to_string(), rgb_to_hex(colors[0]));
    scheme.insert("color15".to_string(), rgb_to_hex(col15));
}

fn getbg_dark(c: Srgb<f32>) -> (Srgb<f32>, Srgb<f32>) {
    let lab: palette::Lab = c.into_color();
    let l = lab.l;
    let a = lab.a * 0.2;
    let b = lab.b * 0.2;

    let (color0_l, bg_l) = if l < 20.0 {
        (l + 20.0, l)
    } else if l < 60.0 {
        (l - 30.0, l - 40.0)
    } else if l < 80.0 {
        (l - 50.0, l - 70.0)
    } else {
        (l - 60.0, l - 80.0)
    };

    let color0_l = color0_l.max(0.0).min(100.0);
    let bg_l = bg_l.max(0.0).min(100.0);

    let color0 = palette::Lab::new(color0_l, a, b);
    let bg = palette::Lab::new(bg_l, a, b);

    let color0_srgb: Srgb<f32> = color0.into_color();
    let bg_srgb: Srgb<f32> = bg.into_color();

    (color0_srgb, bg_srgb)
}

fn getbg_light(c: Srgb<f32>) -> (Srgb<f32>, Srgb<f32>) {
    let lab: palette::Lab = c.into_color();
    let l = lab.l;
    let a = lab.a * 0.2;
    let b = lab.b * 0.2;

    let (color0_l, bg_l) = if l < 20.0 {
        (l + 60.0, l + 70.0)
    } else if l < 60.0 {
        (l + 70.0, l + 60.0)
    } else if l < 80.0 {
        (l + 50.0, l + 30.0)
    } else {
        (l + 40.0, l + 20.0)
    };

    let color0_l = color0_l.max(0.0).min(100.0);
    let bg_l = bg_l.max(0.0).min(100.0);

    let color0 = palette::Lab::new(color0_l, a, b);
    let bg = palette::Lab::new(bg_l, a, b);

    let color0_srgb: Srgb<f32> = color0.into_color();
    let bg_srgb: Srgb<f32> = bg.into_color();

    (color0_srgb, bg_srgb)
}

fn lighten(color: Srgb<f32>, amount: f32) -> Srgb<f32> {
    let lab: palette::Lab = color.into_color();
    let new_l = (lab.l + amount * 100.0).min(100.0);
    let new_lab = palette::Lab::new(new_l, lab.a, lab.b);
    new_lab.into_color()
}

fn darken(color: Srgb<f32>, amount: f32) -> Srgb<f32> {
    let lab: palette::Lab = color.into_color();
    let new_l = (lab.l - amount * 100.0).max(0.0);
    let new_lab = palette::Lab::new(new_l, lab.a, lab.b);
    new_lab.into_color()
}

fn darken_fixed(color: Srgb<f32>, amount: f32) -> Srgb<f32> {
    let lab: palette::Lab = color.into_color();
    let new_l = (lab.l - amount * 100.0).max(0.0);
    let new_lab = palette::Lab::new(new_l, lab.a, lab.b);
    new_lab.into_color()
}

fn blend(a: Srgb<f32>, b: Srgb<f32>) -> Srgb<f32> {
    Srgb::new(
        0.5 * a.red + 0.5 * b.red,
        0.5 * a.green + 0.5 * b.green,
        0.5 * a.blue + 0.5 * b.blue,
    )
}

fn rgb_to_hex(color: Srgb<f32>) -> String {
    let r = (color.red.clamp(0.0, 1.0) * 255.0) as u8;
    let g = (color.green.clamp(0.0, 1.0) * 255.0) as u8;
    let b = (color.blue.clamp(0.0, 1.0) * 255.0) as u8;
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

fn get_cache_dir() -> Result<PathBuf> {
    let cache_dir = dirs::cache_dir()
        .context("Could not find cache directory")?
        .join("wal");
    Ok(cache_dir)
}

fn write_colors_file(cache_dir: &Path, scheme: &HashMap<String, String>) -> Result<()> {
    let colors_file = cache_dir.join("colors");
    let mut file = fs::File::create(&colors_file).context("Failed to create colors file")?;

    for i in 0..16 {
        let key = format!("color{}", i);
        if let Some(color) = scheme.get(&key) {
            writeln!(file, "{}", color)?;
        }
    }

    file.flush()?;
    Ok(())
}

fn write_sequences(cache_dir: &Path, scheme: &HashMap<String, String>) -> Result<()> {
    let sequences_file = cache_dir.join("sequences");
    let mut file = fs::File::create(&sequences_file).context("Failed to create sequences file")?;

    write!(file, "\x1b]4;")?;
    for i in 0..16 {
        let key = format!("color{}", i);
        if let Some(color) = scheme.get(&key) {
            write!(file, "{};{}", i, color)?;
            if i < 15 {
                write!(file, "\\")?;
            }
        }
    }
    write!(file, "\x1b\\")?;

    if let (Some(bg), Some(fg)) = (scheme.get("background"), scheme.get("foreground")) {
        write!(file, "\x1b]11;{}\x1b\\", bg)?;
        write!(file, "\x1b]10;{}\x1b\\", fg)?;
    }

    file.flush()?;
    Ok(())
}

fn apply_terminal_colors(scheme: &HashMap<String, String>) -> Result<()> {
    let sequences = build_ansi_sequences(scheme);

    if let Ok(pts_dir) = fs::read_dir("/dev/pts") {
        for entry in pts_dir.flatten() {
            let path = entry.path();
            if path.to_string_lossy().contains("pts/") {
                if let Ok(mut file) = fs::OpenOptions::new().write(true).open(&path) {
                    let _ = file.write_all(sequences.as_bytes());
                }
            }
        }
    }

    Ok(())
}

fn build_ansi_sequences(scheme: &HashMap<String, String>) -> String {
    let mut seq = String::new();

    for i in 0..16 {
        let key = format!("color{}", i);
        if let Some(color) = scheme.get(&key) {
            seq.push_str(&format!("\x1b]4;{};{}\x1b\\", i, color));
        }
    }

    if let (Some(bg), Some(fg)) = (scheme.get("background"), scheme.get("foreground")) {
        seq.push_str(&format!("\x1b]11;{}\x1b\\", bg));
        seq.push_str(&format!("\x1b]10;{}\x1b\\", fg));
    }

    seq
}

fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    (r, g, b)
}

fn print_palette(scheme: &HashMap<String, String>) {
    println!("\nGenerated Color Palette:\n");

    for i in 0..16 {
        let key = format!("color{}", i);
        if let Some(color) = scheme.get(&key) {
            println!("{}: {}", key, color);
        }
    }

    if let (Some(bg), Some(fg)) = (scheme.get("background"), scheme.get("foreground")) {
        println!("\nBackground: {}", bg);
        println!("Foreground: {}", fg);
    }
    println!();
}
