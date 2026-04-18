<div align="center">

# OptiTux GUI

**A native Linux frontend for managing OptiScaler installations across your game library.**

[![Discord](https://img.shields.io/badge/Discord-Join%20the%20server-5865F2?style=for-the-badge&logo=discord&logoColor=white)](https://discord.com/invite/e5wyA36Zka)
[![Patreon](https://img.shields.io/badge/Patreon-Support-FF424D?style=for-the-badge&logo=patreon&logoColor=white)](https://www.patreon.com/SpexxLorioh)
[![Buy Me a Coffee](https://img.shields.io/badge/Buy%20Me%20a%20Coffee-Support-FFDD00?style=for-the-badge&logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/spexxlorioh)

</div>

---

> [!WARNING]
> **OptiTux GUI is an independent, community-built tool and has no affiliation with the official OptiScaler project.**
> OptiScaler is developed and maintained by its own team. This tool simply provides a graphical interface to manage OptiScaler installations on Linux. All credit for OptiScaler itself goes to its developers — see [Credits](#credits).

---

## What is it?

OptiTux GUI lets you install, configure, and remove [OptiScaler](https://github.com/optiscaler/OptiScaler) across your Linux game library — without touching the terminal or manually copying files.

It scans your Steam and Heroic library, detects your GPU, picks the right upscaling backend automatically, and handles everything from downloading OptiScaler releases to writing the correct configuration into `OptiScaler.ini`.

**What you can do with it:**

- Browse your installed games and see which ones have OptiScaler active
- **Quick Install** — one click, done. Detects your GPU, picks the best upscaler, downloads if needed, installs and configures
- **Custom Install** — choose the exact OptiScaler version, upscaler backend (FSR, DLSS, XeSS), whether to include the FSR4 INT8 addon, and optionally enable Frame Generation
- Download and manage OptiScaler releases from both the official repository and the OptiTuxDB
- Uninstall cleanly, with automatic backup and restore of the original DLLs

---

## Requirements

OptiTux GUI is built with [Tauri](https://tauri.app/) and runs on Linux.

**Runtime dependencies (required):**

- A working desktop environment with WebView support (WebKitGTK)

**Optional but recommended:**

### `p7zip`

OptiScaler releases ship as `.7z` archives. OptiTux GUI includes a fallback extractor, but the native `p7zip` binary is significantly faster — especially noticeable on large archives.

| Distro | Command |
|--------|---------|
| Arch / Manjaro | `sudo pacman -S p7zip` |
| Fedora / RHEL | `sudo dnf install p7zip p7zip-plugins` |
| Ubuntu / Debian | `sudo apt install p7zip-full` |
| openSUSE | `sudo zypper install p7zip` |

### `pciutils` (`lspci`)

Used to accurately detect your GPU model and architecture. Without it, GPU detection falls back to sysfs, which may be less precise for certain cards.

| Distro | Command |
|--------|---------|
| Arch / Manjaro | `sudo pacman -S pciutils` |
| Fedora / RHEL | `sudo dnf install pciutils` |
| Ubuntu / Debian | `sudo apt install pciutils` |
| openSUSE | `sudo zypper install pciutils` |

---

## Installation

### Arch Linux / Manjaro / EndeavourOS / CachyOS
The easiest way to install OptiTux GUI on Arch-based distributions is with our automated script:

```bash
curl -sSL https://raw.githubusercontent.com/Spexxl/OptiTux-GUI/main/install.sh | sudo bash
```

### Debian / Ubuntu / Fedora / Nobara
For other distributions, please download the native package from our [Releases](https://github.com/Spexxl/OptiTux-GUI/releases) page:

- **Debian / Ubuntu / Mint:** Download the [latest .deb](https://github.com/Spexxl/OptiTux-GUI/releases) and install with `sudo apt install ./file.deb`
- **Fedora / RHEL / Nobara:** Download the [latest .rpm](https://github.com/Spexxl/OptiTux-GUI/releases) and install with `sudo dnf install ./file.rpm`

---

## Building from source

You will need [Rust](https://rustup.rs/) and [Node.js](https://nodejs.org/) installed.

```bash
# Clone the repository
git clone https://github.com/Spexxl/OptiTux-GUI.git
cd OptiTux-GUI

# Install frontend dependencies
npm install

# Run in development mode
npm run tauri dev

# Build a release binary
npm run tauri build
```

---

## Support the project

Running a community hub with version hosting and infrastructure costs time and money. If OptiTux GUI has been useful to you, consider supporting its development:

<div align="center">

[![Patreon](https://img.shields.io/badge/Patreon-SpexxLorioh-FF424D?style=for-the-badge&logo=patreon&logoColor=white)](https://www.patreon.com/SpexxLorioh)
[![Buy Me a Coffee](https://img.shields.io/badge/Buy%20Me%20a%20Coffee-spexxlorioh-FFDD00?style=for-the-badge&logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/spexxlorioh)

</div>

---

## Credits

OptiTux GUI would not exist without the work of the OptiScaler project.

**[OptiScaler](https://github.com/optiscaler/OptiScaler)** — the upscaling compatibility layer that OptiTux GUI manages. All upscaling functionality, DLL injection, and INI configuration logic belongs to the OptiScaler team and its contributors.

If you find OptiScaler useful, consider supporting its developers directly through [their repository](https://github.com/optiscaler/OptiScaler).