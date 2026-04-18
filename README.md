# OptiTux GUI 🐧✨

A powerful, modern interface to manage OptiScaler and your game library on Linux with ease.

![OptiTux-GUI Preview](https://github.com/Spexxl/OptiTux-GUI/raw/main/src-tauri/icons/128x128.png)

## 📦 Installation

OptiTux is distributed natively for all major Linux ecosystems. Choose the package that best fits your distribution below.

### 🏹 Arch Linux / CachyOS / Manjaro
For those using Arch-based systems, you have two options:

- **Option 1: Pre-compiled Package (Fastest)**
  Download the `.pkg.tar.zst` from our [Releases](https://github.com/Spexxl/OptiTux-GUI/releases) and run:
  ```bash
  sudo pacman -U ./OptiTux-x86_64.pkg.tar.zst
  ```

- **Option 2: Build with PKGBUILD**
  Clone this repo and run `makepkg -si` in the root folder.

### 🍎 Debian / Ubuntu / Linux Mint / Pop!_OS
1. Download the latest `.deb` package from [Releases](https://github.com/Spexxl/OptiTux-GUI/releases).
2. Install it using your terminal:
   ```bash
   sudo apt install ./OptiTux_amd64.deb
   ```

### 🎩 Fedora / Nobara / RHEL
1. Download the latest `.rpm` package from [Releases](https://github.com/Spexxl/OptiTux-GUI/releases).
2. Install it using DNF:
   ```bash
   sudo dnf install ./OptiTux-x86_64.rpm
   ```

### 📦 Flatpak (Universal)
Works on any distribution with Flatpak support.
1. Download the `.flatpak` bundle from [Releases](https://github.com/Spexxl/OptiTux-GUI/releases).
2. Install it with:
   ```bash
   flatpak install ./OptiTux.flatpak
   ```

---

## 🚀 Key Features
- **OptiScaler Management:** Install and update OptiScaler versions automatically.
- **Game Library:** Scan and organize your Steam, GOG, and Epic games.
- **DPI Scaling:** Full support for modern high-DPI monitors.
- **Native Experience:** No more AppImage/Sandbox issues, full system integration.

---

## 🛠 Building from source
Requires [Node.js](https://nodejs.org/) and [Rust](https://www.rust-lang.org/).

```bash
npm install
npm run tauri build
```

---

## 📄 License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.