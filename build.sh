#!/usr/bin/env bash
# Build script for the devtools desktop app.
#
# Usage:
#   ./build.sh              # release build (default)
#   ./build.sh debug        # debug build
#   ./build.sh release      # release build (explicit)
#   ./build.sh run          # build + run (release)
#   ./build.sh clean        # cargo clean
#   ./build.sh dist         # release + strip + copy to ./dist/

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$PROJECT_ROOT"

MODE="${1:-release}"

require_rust() {
    if ! command -v cargo >/dev/null 2>&1; then
        echo "error: cargo not found in PATH" >&2
        echo "install Rust via https://rustup.rs and re-run this script." >&2
        exit 1
    fi
}

case "$MODE" in
    debug)
        require_rust
        echo "==> cargo build (debug)"
        cargo build
        echo "==> built: target/debug/devtools"
        ;;

    release)
        require_rust
        echo "==> cargo build --release"
        cargo build --release
        echo "==> built: target/release/devtools"
        ;;

    run)
        require_rust
        echo "==> cargo run --release"
        exec cargo run --release
        ;;

    clean)
        require_rust
        echo "==> cargo clean"
        cargo clean
        rm -rf "$PROJECT_ROOT/dist"
        echo "==> cleaned target/ and dist/"
        ;;

    dist)
        require_rust
        DIST_DIR="$PROJECT_ROOT/dist"
        echo "==> cargo build --release"
        cargo build --release
        mkdir -p "$DIST_DIR"
        cp target/release/devtools "$DIST_DIR/devtools"
        if command -v strip >/dev/null 2>&1; then
            echo "==> strip"
            strip "$DIST_DIR/devtools"
        fi
        SIZE=$(du -h "$DIST_DIR/devtools" | cut -f1)
        echo "==> dist: $DIST_DIR/devtools ($SIZE)"
        ;;

    -h|--help|help)
        sed -n '2,11p' "$0"
        ;;

    *)
        echo "error: unknown mode '$MODE'" >&2
        echo "valid modes: debug | release | run | clean | dist" >&2
        exit 2
        ;;
esac
