# OptiTux-GUI Architecture & Implementation Plan

## 1. Project Overview
OptiTux-GUI is a graphical tool designed to manage and simplify the installation of OptiScaler (an AI upscaling drop-in replacing DLSS/FSR/XeSS) on Linux. The application is built using Rust and GTK4 (via libadwaita for a native GNOME look), and will be packaged as an AppImage for universal Linux distribution.

## 2. Technology Stack
*   **Core:** Rust
*   **GUI Framework:** GTK4 with `libadwaita` (via `gtk4-rs` and `libadwaita-rs` bindings)
*   **Packaging:** AppImage (via `linuxdeploy` and GitHub Actions)
*   **Async Runtime / Networking:** `tokio` and `reqwest` (for fetching OptiScaler releases from GitHub)
*   **Archive Extraction:** `zip` / `tar` (to extract the OptiScaler downloaded archives)

## 3. Architecture Context
We use a clean separation between the GTK UI layer and the core processing logic.

*   **Core (`src/core/`)**: Handles game detection (Steam, Heroic, Lutris environments), fetching the latest OptiScaler releases, and the file operations to patch the game binaries or place the DLLs next to the executable.
*   **UI (`src/ui/`)**: Contains the GTK views — a Main Window split into a Game List page and a Settings/Logs page.
*   **Config (`src/config/`)**: Manages the application's user configuration state (paths, selected default versions) using `serde` to persist to `~/.config/optitux-gui`.

## 4. Directory Structure
```text
OptiTux-GUI/
├── Cargo.toml               # Rust dependencies and project metadata
├── ARCHITECTURE.md          # This file
├── build.rs                 # Build script for glib resources/translations (future)
├── src/
│   ├── main.rs              # Application entry point
│   ├── app.rs               # Main application state and initialization
│   ├── ui/                  # User Interface modules
│   │   ├── mod.rs
│   │   ├── main_window.rs   # The primary libadwaita window
│   │   ├── game_list.rs     # Widget to display detected games
│   │   └── installer.rs     # Modal/Dialog for downloading and progress
│   ├── core/                # Business logic
│   │   ├── mod.rs
│   │   ├── optiscaler.rs    # OptiScaler GitHub API interaction
│   │   ├── game_scanner.rs  # Locating games via standard launcher paths
│   │   └── installer.rs     # Logic to copy DLLs and configure wine/proton
│   └── config/              # Settings
│       └── mod.rs           # State serialization
├── data/                    # App metadata for desktop integration
│   ├── io.github.optitux.desktop
│   ├── io.github.optitux.metainfo.xml
│   └── icons/               # Hi-res and symbolic icons
└── packaging/
    ├── AppDir/              # Skeleton for AppImage
    └── build_appimage.sh    # Script to bundle the binary and dependencies
```

## 5. Key Dependencies (`Cargo.toml`)
*   `gtk4`, `libadwaita` - UI Framework
*   `tokio` - Asynchronous runtime
*   `reqwest` - HTTP Client for GitHub API
*   `serde`, `serde_json` - Configuration parsing
*   `directories` - Resolving OS standard paths (`~/.config`, `~/.local/share`)
*   `zip`, `anyhow` - Archive handling and Error handling

## 6. AppImage Packaging Strategy
To create the `.AppImage` file, the build process will:
1. Compile the Rust binary in release mode (`cargo build --release`).
2. Copy the binary, assets, and GTK/GNOME dependencies into an `AppDir` folder using `linuxdeploy`.
3. Use the `linuxdeploy-plugin-gtk` to securely bundle standard GTK schemas, icons, and themes.
4. Convert the `AppDir` into the final `OptiTux-GUI-x86_64.AppImage`.
