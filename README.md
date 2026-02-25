# cosmic-ext-ip-applet
A COSMIC panel applet that displays your IPv4 addresses directly in the panel.  
This project is _**heavily vibe coded**_ so if you are allergic to that, stay away from this.

<img width="578" height="313" alt="image" src="https://github.com/user-attachments/assets/88f7a91c-4d34-4294-b4c4-bce5eefd0be4" />
<img width="567" height="510" alt="image" src="https://github.com/user-attachments/assets/d753a547-9108-4ab4-9451-089027e127a6" />


## Features

- **Automatically detects all** active network interfaces with an IPv4 address.
- **Local IP addresses displayed** per interface (`eth0`, `wlan0` etc.) and only shown if active.
- **Public IP address** fetched from your choice of service.
- **Easy to copy IP address** to the clipboard.
- **Auto-refresh** every 10 seconds (configurable).
- **Built-in settings:**
  - Toggle which interfaces to display.
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

## Uninstallation

```bash
./uninstall.sh
```

## Troubleshooting
**Build fails with "libcosmic not found":**  
Ensure you have all build-dependencies installed (see above). `libcosmic` is downloaded automatically from GitHub during the build.

**Applet is not visible in the list:**  
Verify that the .desktop file has been copied: `ls /usr/share/applications/io.github.Pontus-Ottosson.CosmicIpApplet.desktop`  
And this should be the contents of the file:

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

**Public IP shows as "Unavailable":**  
Check your internet connection. Try switching to a different service in the settings.
