# 🌐 cosmic-ip-applet

En COSMIC-panel-applet som visar dina IPv4-adresser direkt i panelen.

## Funktioner

- **Lokala IP-adresser** för `eth0`, `wlan0` och `tun0` (visas bara om de är aktiva)
- **Publik IP-adress** hämtad från valfri tjänst
- **Automatisk uppdatering** var 10:e sekund (konfigurerbart)
- **Klickbar popup** med detaljerad info och inställningar
- **Inbyggda inställningar:**
  - Välj vilka gränssnitt som ska visas
  - Välj textfärg (vit, grön, cyan, gul, orange, röd eller standard)
  - Välj tjänst för publik IP (ifconfig.io, ipify.org, m.fl.)
  - Välj uppdateringsintervall (5, 10, 15, 30 eller 60 sekunder)

---

## Förutsättningar

Du behöver ha följande installerat:

1. **Rust & Cargo** – installeras via [rustup.rs](https://rustup.rs):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

2. **libcosmic build-dependencies** (Ubuntu/Pop!_OS):
   ```bash
   sudo apt install libwayland-dev libxkbcommon-dev libseat-dev \
       libinput-dev mesa-common-dev libgles2-mesa-dev \
       libudev-dev libdbus-1-dev pkg-config cmake \
       libpipewire-0.3-dev libpulse-dev just
   ```

   På Arch Linux:
   ```bash
   sudo pacman -S wayland libxkbcommon libseat libinput mesa \
       udev dbus pkgconf cmake pipewire libpulse just
   ```

---

## Installation

```bash
# Klona eller ladda ner projektet, gå in i mappen, kör sedan:
chmod +x install.sh
./install.sh
```

Skriptet bygger projektet och installerar det automatiskt.

Här är en färdig .desktop fil som ska ligga här /usr/share/applications/com.example.CosmicIpApplet.desktop:

```bash
[Desktop Entry]
Name=IP Applet
Comment=Visar nätverks-IP i panelen
Exec=cosmic-ip-applet
Icon=network-wired-symbolic
Terminal=false
Type=Application
Categories=Cosmic;Applet;
X-CosmicApplet=true
```


---

## Lägga till i COSMIC-panelen

1. Högerklicka på panelen
2. Välj **Panel-inställningar**
3. Gå till fliken **Applets**
4. Hitta **IP Applet** och klicka på **+** för att lägga till den

---

## Användning

- **Klicka** på appleten för att öppna popup-fönstret
- I popup:
  - Fliken **📡 IP-adresser** visar aktuella adresser
  - Fliken **⚙ Inställningar** låter dig justera allt

---

## Avinstallation

```bash
./uninstall.sh
```

---

## Felsökning

**Bygget misslyckas med "libcosmic not found":**
Se till att du har alla build-dependencies installerade (se ovan). libcosmic laddas ned automatiskt från GitHub under bygget.

**Appleten syns inte i listan:**
Kontrollera att `.desktop`-filen har kopierats: `ls /usr/share/applications/com.example.CosmicIpApplet.desktop`

**Publik IP visas som "Ej tillgänglig":**
Kontrollera din internetanslutning. Prova att byta tjänst i inställningarna.

**Lägga till fler gränssnitt (t.ex. eth1, wlan1):**
Öppna `src/main.rs` och hitta raden:
```rust
const KNOWN_INTERFACES: &[&str] = &["eth0", "wlan0", "tun0"];
```
Lägg till ditt gränssnitt där, t.ex. `"eth1"`, och bygg om med `./install.sh`.
