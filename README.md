# cosmic-ip-applet
A COSMIC panel applet that displays your IPv4 addresses directly in the panel.  
This project is _**heavily vibe coded**_ so if you are allergic to that, stay away from this.

<img width="555" height="300" alt="image" src="https://github.com/user-attachments/assets/ad680334-6cd9-4274-848f-9d1399990086" />
<img width="557" height="505" alt="image" src="https://github.com/user-attachments/assets/73ed0f91-8e9d-47be-87c8-f9e7ebc7678e" />

## Features

- **Local IP addresses** for `eth0`, `wlan0`, and `tun0` (only displayed if active).
- **Public IP address** fetched from your choice of service.
- **Auto-refresh** every 10 seconds (configurable).
- **Clickable popup** with detailed info and settings.
- **Built-in settings:**
  - Select which interfaces to display.
  - Choose text color (White, Green, Cyan, Yellow, Orange, Red, or Default).
  - Select service for Public IP (ifconfig.io, ipify.org, etc.).
  - Set update interval (5, 10, 15, 30, or 60 seconds).

---

## Prerequisites

You need to have the following installed:

1. **Rust & Cargo** – installed via [rustup.rs](https://rustup.rs):

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
    source $HOME/.cargo/env
    ```

2. **libcosmic build**-dependencies (Ubuntu/Pop!_OS):
    ```bash
    sudo apt install libwayland-dev libxkbcommon-dev libseat-dev \
    libinput-dev mesa-common-dev libgles2-mesa-dev \
    libudev-dev libdbus-1-dev pkg-config cmake \
    libpipewire-0.3-dev libpulse-dev just
    ```
    
    On Arch Linux:
  
    ```bash
    sudo pacman -S wayland libxkbcommon libseat libinput mesa \
    udev dbus pkgconf cmake pipewire libpulse just
    ```

## Installation

```bash
# Clone or download the project, enter the folder, then run:
chmod +x install.sh
./install.sh
```
The script builds the project and installs it automatically.  
Should you run into trouble with the applet not showing up, make sure that the .desktop file is correct.  
The following `.desktop` file should be located at `/usr/share/applications/com.example.CosmicIpApplet.desktop:`
```bash
[Desktop Entry]
Name=IP Applet
Comment=Shows IP addresses in COSMIC panel
Exec=cosmic-ip-applet
Icon=network-wired-symbolic
Terminal=false
Type=Application
Categories=Cosmic;Applet;
X-CosmicApplet=true
```

## Uninstallation

```bash
./uninstall.sh
```

## Troubleshooting
**Build fails with "libcosmic not found":**  
Ensure you have all build-dependencies installed (see above). `libcosmic` is downloaded automatically from GitHub during the build.

**Applet is not visible in the list:**  
Verify that the .desktop file has been copied: `ls /usr/share/applications/com.example.CosmicIpApplet.desktop`

**Public IP shows as "Unavailable":**  
Check your internet connection. Try switching to a different service in the settings.

**Adding more interfaces (e.g., eth1, wlan1):**  
Open `src/main.rs` and find the line:

```rust
const KNOWN_INTERFACES: &[&str] = &["eth0", "wlan0", "tun0"];
```

Add your interface there, e.g., `eth1`, and rebuild with `./install.sh`.
