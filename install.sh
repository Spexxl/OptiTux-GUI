#!/bin/bash

# OptiTux Universal Installer
# Detects distribution and installs the appropriate native package

set -e

# Colors for better output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}==>${NC} Starting OptiTux GUI installation..."

# Ensure we are running as root
if [ "$EUID" -ne 0 ]; then 
  echo -e "${RED}Error:${NC} Please run as root (use sudo)."
  exit 1
fi

# Detect Distribution
if [ -f /etc/os-release ]; then
    . /etc/os-release
    OS=$ID
    VER=$VERSION_ID
else
    echo -e "${RED}Error:${NC} Could not detect your Linux distribution."
    exit 1
fi

# Get the latest version from GitHub API
VERSION=$(curl -s https://api.github.com/repos/Spexxl/OptiTux-GUI/releases/latest | grep '"tag_name":' | sed -E 's/.*"v([^"]+)".*/\1/')

if [ -z "$VERSION" ]; then
    VERSION="0.1.0" # Fallback
fi

echo -e "${BLUE}==>${NC} Detected OS: ${GREEN}$OS${NC}"
echo -e "${BLUE}==>${NC} Latest Version: ${GREEN}v$VERSION${NC}"

case $OS in
    ubuntu|debian|linuxmint|pop|zoris)
        echo -e "${BLUE}==>${NC} Installing for Debian-based system..."
        FILE="OptiTux_${VERSION}_amd64.deb"
        URL="https://github.com/Spexxl/OptiTux-GUI/releases/download/v${VERSION}/${FILE}"
        
        apt-get update
        apt-get install -y wget webkit2gtk-4.1 libappindicator3-1 librsvg2-common
        wget -q $URL -O /tmp/$FILE
        apt-get install -y /tmp/$FILE
        rm /tmp/$FILE
        ;;

    fedora|nobara|cachyos)
        # Note: Some Arch-based like CachyOS might have ID=cachyos but can use RPM if configured, 
        # but let's stick to true RPM distros first.
        echo -e "${BLUE}==>${NC} Installing for Fedora-based system..."
        FILE="OptiTux-${VERSION}-1.x86_64.rpm"
        URL="https://github.com/Spexxl/OptiTux-GUI/releases/download/v${VERSION}/${FILE}"
        
        dnf install -y wget webkit2gtk4.1 libappindicator-gtk3 librsvg2
        wget -q $URL -O /tmp/$FILE
        dnf install -y /tmp/$FILE
        rm /tmp/$FILE
        ;;

    arch|manjaro|endeavouros)
        echo -e "${BLUE}==>${NC} Installing for Arch-based system..."
        # On Arch, the clean way is to use the PKGBUILD. We will do a quick manual build.
        # But since we need to run as non-root for makepkg, we handle dependencies as root 
        # and then use a small trick.
        
        pacman -Sy --needed --noconfirm webkit2gtk-4.1 libappindicator-gtk3 librsvg wget base-devel
        
        USER_HOME=$(getent passwd $SUDO_USER | cut -d: -f6)
        sudo -u $SUDO_USER bash <<EOF
            cd /tmp
            wget -q https://raw.githubusercontent.com/Spexxl/OptiTux-GUI/main/PKGBUILD -O PKGBUILD
            # Update version in temporal PKGBUILD if needed
            sed -i "s/pkgver=.*/pkgver=$VERSION/" PKGBUILD
            makepkg -si --noconfirm
            rm PKGBUILD
EOF
        ;;

    *)
        echo -e "${RED}Error:${NC} Your distribution ($OS) is not supported by this script yet."
        echo "Please download the package manually from GitHub."
        exit 1
        ;;
esac

echo -e "${GREEN}==> OptiTux GUI has been installed successfully!${NC}"
echo "You can now find OptiTux in your application menu."
