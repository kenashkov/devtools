#!/usr/bin/env bash
# Install DevTools as a desktop app on Linux (user-local, no sudo).
#
# Usage:
#   ./packaging/install-linux.sh             # build release + install for current user
#   ./packaging/install-linux.sh --uninstall # remove user-local install

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN_DIR="$HOME/.local/bin"
APP_DIR="$HOME/.local/share/applications"
ICON_DIR="$HOME/.local/share/icons/hicolor/scalable/apps"

case "${1:-install}" in
    --uninstall|uninstall)
        rm -f "$BIN_DIR/devtools"
        rm -f "$APP_DIR/devtools.desktop"
        rm -f "$ICON_DIR/devtools.svg"
        if command -v update-desktop-database >/dev/null 2>&1; then
            update-desktop-database "$APP_DIR" >/dev/null 2>&1 || true
        fi
        echo "uninstalled."
        exit 0
        ;;
esac

cd "$PROJECT_ROOT"
echo "==> building release"
cargo build --release

mkdir -p "$BIN_DIR" "$APP_DIR" "$ICON_DIR"

install -m 755 target/release/devtools "$BIN_DIR/devtools"
install -m 644 packaging/devtools.desktop "$APP_DIR/devtools.desktop"
install -m 644 assets/icon.svg "$ICON_DIR/devtools.svg"

if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database "$APP_DIR" >/dev/null 2>&1 || true
fi
if command -v gtk-update-icon-cache >/dev/null 2>&1; then
    gtk-update-icon-cache -t "$HOME/.local/share/icons/hicolor" >/dev/null 2>&1 || true
fi

echo "==> installed:"
echo "    binary  $BIN_DIR/devtools"
echo "    desktop $APP_DIR/devtools.desktop"
echo "    icon    $ICON_DIR/devtools.svg"
echo "(make sure $BIN_DIR is on your PATH)"
