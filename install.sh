#!/bin/bash

# OptiTux Installer for Arch Linux / Manjaro / EndeavourOS / CachyOS
# This script automates the installation using the PKGBUILD

set -e

# Colors for better output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}==>${NC} Starting OptiTux GUI installation for Arch-based system..."

# Ensure we are running as root to install dependencies
if [ "$EUID" -ne 0 ]; then 
  echo -e "${RED}Error:${NC} Please run as root (use sudo)."
  exit 1
fi

# Get the latest version from GitHub API with a robust fallback
VERSION=$(curl -s https://api.github.com/repos/Spexxl/OptiTux-GUI/releases/latest | grep '"tag_name":' | head -n 1 | cut -d '"' -f 4 | sed 's/^v//' || echo "0.1.0")

# Final check to ensure VERSION is not empty or invalid
if [[ ! "$VERSION" =~ ^[0-9] ]]; then
    VERSION="0.1.0"
fi

echo -e "${BLUE}==>${NC} Latest Version: ${GREEN}v$VERSION${NC}"

# Install dependencies via pacman
echo -e "${BLUE}==>${NC} Installing dependencies..."
pacman -Sy --needed --noconfirm webkit2gtk-4.1 libappindicator-gtk3 librsvg wget base-devel

# Use makepkg (must be run as non-root user)
USER_HOME=$(getent passwd $SUDO_USER | cut -d: -f6)
echo -e "${BLUE}==>${NC} Building and installing package..."

sudo -u $SUDO_USER bash <<EOF
    cd /tmp
    wget -q https://raw.githubusercontent.com/Spexxl/OptiTux-GUI/main/PKGBUILD -O PKGBUILD
    # Update version in temporal PKGBUILD if needed
    sed -i "s/pkgver=.*/pkgver=$VERSION/" PKGBUILD
    makepkg -si --noconfirm
    rm PKGBUILD
EOF

echo -e "${GREEN}==> OptiTux GUI has been installed successfully!${NC}"
echo "You can now find OptiTux in your application menu."
