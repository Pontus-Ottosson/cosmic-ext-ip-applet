#!/usr/bin/env bash
set -e

echo "=== Uninstalling cosmic-ip-applet ==="
sudo rm -f /usr/bin/cosmic-ip-applet
sudo rm -f /usr/share/applications/io.github.Pontus-Ottosson.CosmicIpApplet.desktop
echo "Done!"
