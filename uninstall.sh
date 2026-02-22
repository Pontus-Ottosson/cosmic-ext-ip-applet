#!/usr/bin/env bash
set -e

echo "=== Avinstallerar cosmic-ip-applet ==="
sudo rm -f /usr/bin/cosmic-ip-applet
sudo rm -f /usr/share/applications/com.example.CosmicIpApplet.desktop
echo "Klart!"
