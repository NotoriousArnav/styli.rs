# styli-rs

Fast *nix wallpaper switcher with built-in color scheme generation.

## Features

- Download wallpapers from Picsum, Unsplash, Reddit, DeviantArt, or local directories
- Set wallpapers on any desktop environment (feh, nitrogen, GNOME, KDE, Sway, Hyprland, SWWW)
- Generate 16-color palettes using internal k-means clustering (wallust-compatible)
- Full `wal`/`wallust`/`pywal16` CLI compatibility via `styli-rs wal`
- Configurable via `~/.config/styli.toml`
- Custom backend support via shell command templates

## Installation

```bash
# Download the latest release for your architecture
curl -sL https://github.com/NotoriousArnav/styli.rs/releases/latest | tar xz
chmod +x styli-rs
sudo mv styli-rs /usr/local/bin/

# Or build from source
cargo build --release
sudo cp target/release/styli-rs /usr/local/bin/
```

## Usage

### Set Wallpaper

```bash
# Fetch and set a random wallpaper
styli-rs set

# Or just
styli-rs

# Specify source and resolution
styli-rs set -s reddit -r 1920x1080

# Use a different wallpaper setter
styli-rs set --backend nitrogen --mode tile

# Skip color generation
styli-rs set --no-colors
```

### Color Generation (wallust/pywal16 compatible)

```bash
# Generate colors from an image
styli-rs wal wallpaper.jpg

# Light color palette
styli-rs wal wallpaper.jpg --light
styli-rs wal wallpaper.jpg -l

# Preview colors
styli-rs wal wallpaper.jpg --preview

# Skip terminal colors
styli-rs wal wallpaper.jpg --skip-terminal

# Restore previous colors
styli-rs wal --restore
styli-rs wal -R

# Quiet mode
styli-rs wal wallpaper.jpg -q
```

Full wallust/pywal16 compatibility - most flags work:
- `-i, --image` - Image file
- `-l, --light` - Light palette
- `-s, --skip-terminal` - Skip terminal colors
- `-n, --skip-wallpaper` - Skip wallpaper
- `-q, --quiet` - Quiet mode
- `-p, --preview` - Preview colors
- `-w, --overwrite-cache` - Force regenerate
- `-b, --backend` - Extraction backend
- `-c, --colorspace` - Colorspace (lab, lch)
- `--palette` - Palette scheme (dark, light, dark16)
- `--saturation` - Saturation level
- `--cols16` - 16 color output (pywal16 compat)
- `-R, --restore` - Restore previous
- `-t, --skip-tty` - Skip TTY
- `-e, --skip-reload` - Skip reload

## Configuration

Create `~/.config/styli.toml`:

```toml
[wallpaper]
source = "picsum"
resolution = "auto"
backend = "feh"
bgtype = "fill"

[custom]
command = "feh --bg-{bgtype} {wallpaper}"

[colors]
enabled = true
cols16 = true
backend = "kmeans"

[reddit]
subreddits = ["wallpapers", "earthporn", "nature"]
sort = "hot"
```

## Custom Backend

Use shell commands with template variables:

```toml
[wallpaper]
backend = "custom"

[custom]
command = "feh --bg-{bgtype} {wallpaper} && notify-send 'styli-rs' 'Wallpaper set!'"
```

Variables: `{wallpaper}`, `{resolution}`, `{monitor}`, `{bgtype}`

## Requirements

- A Wayland or X11 compositor
- For wallpaper setting: feh, nitrogen, gsettings, qdbus, or similar

## Credits

- Original styli: [github.com/thevinter/styli.sh](https://github.com/thevinter/styli.sh)
- Color extraction based on [wallust](https://codeberg.org/explosion-mental/wallust) algorithms
- Built with [kmeans_colors](https://github.com/okaneco/kmeans-colors) crate

## License

MIT License - see [LICENSE](LICENSE) file.
