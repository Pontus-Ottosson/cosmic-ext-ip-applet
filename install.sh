#!/usr/bin/env bash
# Installations-skript för cosmic-ip-applet
# Kör som vanlig användare (inte root), sudo används automatiskt vid behov.

set -e

BINARY_NAME="cosmic-ip-applet"
APP_ID="com.example.CosmicIpApplet"

echo "=== Bygger cosmic-ip-applet (release) ==="
cargo build --release

echo ""
echo "=== Installerar binär ==="
sudo install -Dm755 "target/release/$BINARY_NAME" "/usr/bin/$BINARY_NAME"

echo "=== Installerar .desktop-fil ==="
sudo install -Dm644 "data/$APP_ID.desktop" "/usr/share/applications/$APP_ID.desktop"

echo ""
echo "=== Klart! ==="
echo ""
echo "För att lägga till appleten i COSMIC-panelen:"
echo "  1. Högerklicka på panelen"
echo "  2. Välj 'Panel-inställningar' → 'Applets'"
echo "  3. Hitta 'IP Applet' i listan och lägg till den"
echo ""
echo "Tips: Om du vill avinstallera, kör ./uninstall.sh"
