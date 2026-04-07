# styli-rs

Fast *nix wallpaper switcher with wallust color integration.

## Features

- Download wallpapers from Picsum, Unsplash, Reddit, DeviantArt, or local directories
- Set wallpapers on any desktop environment (feh, nitrogen, GNOME, KDE, Sway, Hyprland, SWWW)
- Generate 16-color palettes with wallust
- Configurable via `~/.config/styli.toml`
- Custom backend support via shell command templates
- Static binaries for x86_64, i686, aarch64, and armv7

## Installation

```bash
# Download the latest release for your architecture
curl -sL https://github.com/NotoriousArnav/styli.rs/releases/latest | tar xz
chmod +x styli-rs
sudo mv styli-rs /usr/local/bin/

# Or use install.sh
curl -sL https://raw.githubusercontent.com/NotoriousArnav/styli.rs/main/old/install.sh | bash
```

## Usage

```bash
# Fetch and set a random wallpaper
styli-rs

# Specify source and resolution
styli-rs -s reddit -r 1920x1080

# Skip color generation
styli-rs --no-colors

# Use a different wallpaper setter
styli-rs --backend nitrogen --mode tile
```

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
- For colors: wallust (`cargo install wallust`)
