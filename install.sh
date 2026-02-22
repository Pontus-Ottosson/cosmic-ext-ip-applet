#!/usr/bin/env bash
# Installation script for cosmic-ip-applet
# To be run as normal user, sudo permissions called for when needed

set -e

BINARY_NAME="cosmic-ip-applet"
APP_ID="com.example.CosmicIpApplet"

echo "=== Bygger cosmic-ip-applet (release) ==="
cargo build --release

echo ""
echo "=== Installing binary ==="
sudo install -Dm755 "target/release/$BINARY_NAME" "/usr/bin/$BINARY_NAME"

echo "=== Installing .desktop-file ==="
sudo install -Dm644 "data/$APP_ID.desktop" "/usr/share/applications/$APP_ID.desktop"

echo ""
echo "=== Done! ==="
echo ""
echo "To uninstall, run ./uninstall.sh"
