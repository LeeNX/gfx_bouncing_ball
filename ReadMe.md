# gfx\_bouncing\_ball

Learning experiment for building a Rust-based SDL2 application.

---

## ✅ Supported Native Builds

The project should build natively on:

* `aarch64-unknown-linux-gnu`
* `x86_64-unknown-linux-gnu`
* `riscv64gc-unknown-linux-gnu`
* `aarch64-apple-darwin`
* `x86_64-apple-darwin`

---

## 🔄 Cross-Compiling

* **Linux targets:** Tested using [cross-rs](https://github.com/cross-rs/cross) with Docker/Podman.
* **macOS targets:** Cross-compilation works only when building from a macOS host.
* **Windows targets:** Work in progress — currently failing for both cross-compilation and native builds.

Example cross builds:

```bash
cross build --target aarch64-unknown-linux-gnu
cross build --target x86_64-unknown-linux-gnu
cross build --target riscv64gc-unknown-linux-gnu
cross build --target x86_64-pc-windows-gnu --release
```

For Podman users:

```bash
export CROSS_CONTAINER_ENGINE_NO_BUILDKIT=1
export CROSS_CONTAINER_ENGINE=podman
export HTTP_PROXY=http://192.168.101.10:3142

cross build --target aarch64-unknown-linux-gnu
```

---

## 📦 Dependencies

### macOS

Using **Homebrew**:

```bash
brew install sdl2 sdl2_ttf freetype sdl2_image sdl2_mixer sdl2_gfx
```

Using **MacPorts**:

```bash
port install libsdl2 sdl2_ttf sdl2_image sdl2_mixer sdl2_gfx
```

### Debian/Ubuntu

Build/dev packages:

```bash
sudo apt install libsdl2-dev libsdl2-ttf-dev libsdl2-image-dev libsdl2-mixer-dev libsdl2-gfx-dev
```

Runtime packages:

```bash
sudo apt install libsdl2-2.0-0 libsdl2-ttf-2.0-0 libsdl2-image-2.0-0 libsdl2-mixer-2.0-0 libsdl2-gfx-1.0-0
```

### Arch Linux *(untested)*

```bash
sudo pacman -S sdl2 sdl2_ttf sdl2_image sdl2_mixer sdl2_gfx
```

### Fedora *(untested)*

```bash
sudo dnf install SDL2-devel SDL2_ttf-devel SDL2_image-devel SDL2_mixer-devel SDL2_gfx-devel
```

### Windows *(via vcpkg)*

Using [vcpkg](https://github.com/microsoft/vcpkg):

```powershell
vcpkg.exe install sdl2:x64-windows sdl2-ttf:x64-windows sdl2-image:x64-windows sdl2-mixer:x64-windows sdl2-gfx:x64-windows
```

---

## ▶️ Build Instructions

### Native Build

```bash
cargo build --release
```

### Example: macOS cross-compile to x86\_64

```bash
cargo build --release --target x86_64-apple-darwin
```

---

## 🖥️ Running Without X11/Wayland (Linux)

For Linux framebuffers:

```bash
export SDL_VIDEODRIVER=fbcon
```

---

## 🖋️ Font

This project uses the **Ubuntu Mono** font, available from Google Fonts:
👉 [Ubuntu Mono](https://fonts.google.com/specimen/Ubuntu+Mono)

---

## 🚧 ToDo / Known Issues

* Using Linux Framebuffer reports and run, but still runs
  `error: XDG_RUNTIME_DIR is invalid or not set in the environment.`
* Windows build issues

---

## GitHub Actions CI

This project is built automatically on Linux, macOS, and Windows using a build matrix in GitHub Actions. You can download build artifacts from the [Actions page](https://github.com/LeeNX/gfx_bouncing_ball/actions).

---

## License

MIT
