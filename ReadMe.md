CROSS_CONTAINER_ENGINE_NO_BUILDKIT=1
CROSS_CONTAINER_ENGINE=podman

# OSX
```bash
brew install sdl2 sdl2_ttf freetype sdl2_image
```
or
```bash
port install libsdl2 sdl2_ttf sdl2_image
```

# Debian/Ubuntu (check actual version)
## Build/Dev packages installs
```bash
sudo apt install libsdl2-dev libsdl2-image-dev
```
## Runtime needed packages
```bash
sudo apt install libsdl2-2.0-0 libsdl2-image-2.0-0 libsdl2-ttf-2.0-0
```

# Arch
```bash
sudo pacman -S sdl2
```

# Fedora
```bash
sudo dnf install SDL2-devel
```

```bash
cargo build --release
```

# Cross-compile

```bash
cargo build --release --target x86_64-apple-darwin
```

## for podman
```bash
export CROSS_CONTAINER_ENGINE_NO_BUILDKIT=1
export CROSS_CONTAINER_ENGINE=podman
export HTTP_PROXY=http://192.168.101.10:3142

cross build --target aarch64-unknown-linux-gnu
or
cross build --target x86_64-unknown-linux-gnu
```

```
cross build --target aarch64-unknown-linux-gnu
cross build --target x86_64-unknown-linux-gnu
cross build --target x86_64-pc-windows-gnu --release
```

# Use framebuffer video driver
```bash
export SDL_VIDEODRIVER=fbcon
```

# Font
Using Ubuntu Mono font, can be found at
https://fonts.google.com/specimen/Ubuntu+Mono

# ToDo
## Investigate
```bash
leet@cylon:~/src/gfx_bouncing_ball$ sudo ./gfx_bouncing_ball
error: XDG_RUNTIME_DIR is invalid or not set in the environment.
```
