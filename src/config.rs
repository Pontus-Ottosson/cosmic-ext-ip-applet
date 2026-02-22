use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub enabled_interfaces: HashSet<String>,
    pub show_public_ip: bool,
    pub public_ip_service: PublicIpService,
    pub refresh_rate_secs: u64,
    pub text_color: TextColor,
}

impl Default for Config {
    fn default() -> Self {
        let mut enabled = HashSet::new();
        enabled.insert("eth0".to_string());
        enabled.insert("wlan0".to_string());
        enabled.insert("tun0".to_string());

        Self {
            enabled_interfaces: enabled,
            show_public_ip: true,
            public_ip_service: PublicIpService::Ifconfig,
            refresh_rate_secs: 10,
            text_color: TextColor::Default,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublicIpService {
    Ifconfig,
    IpifyOrg,
    ApiIpifyOrg,
    IcanhazmyipCom,
}

impl PublicIpService {
    pub fn url(&self) -> &str {
        match self {
            PublicIpService::Ifconfig => "https://ifconfig.io/ip",
            PublicIpService::IpifyOrg => "https://api.ipify.org",
            PublicIpService::ApiIpifyOrg => "https://api4.ipify.org",
            PublicIpService::IcanhazmyipCom => "https://icanhazmyip.com/ip",
        }
    }

    pub fn label(&self) -> &str {
        match self {
            PublicIpService::Ifconfig => "ifconfig.io",
            PublicIpService::IpifyOrg => "ipify.org",
            PublicIpService::ApiIpifyOrg => "api4.ipify.org",
            PublicIpService::IcanhazmyipCom => "icanhazmyip.com",
        }
    }

    pub fn all() -> Vec<PublicIpService> {
        vec![
            PublicIpService::Ifconfig,
            PublicIpService::IpifyOrg,
            PublicIpService::ApiIpifyOrg,
            PublicIpService::IcanhazmyipCom,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextColor {
    Default,
    White,
    Green,
    Cyan,
    Yellow,
    Orange,
    Red,
}

impl TextColor {
    pub fn label(&self) -> &str {
        match self {
            TextColor::Default => "Standard (tema)",
            TextColor::White => "Vit",
            TextColor::Green => "Grön",
            TextColor::Cyan => "Cyan",
            TextColor::Yellow => "Gul",
            TextColor::Orange => "Orange",
            TextColor::Red => "Röd",
        }
    }

    pub fn all() -> Vec<TextColor> {
        vec![
            TextColor::Default,
            TextColor::White,
            TextColor::Green,
            TextColor::Cyan,
            TextColor::Yellow,
            TextColor::Orange,
            TextColor::Red,
        ]
    }

    pub fn to_iced_color(&self) -> Option<cosmic::iced::Color> {
        match self {
            TextColor::Default => None,
            TextColor::White => Some(cosmic::iced::Color::WHITE),
            TextColor::Green => Some(cosmic::iced::Color::from_rgb(0.0, 0.85, 0.3)),
            TextColor::Cyan => Some(cosmic::iced::Color::from_rgb(0.0, 0.85, 0.85)),
            TextColor::Yellow => Some(cosmic::iced::Color::from_rgb(1.0, 0.85, 0.0)),
            TextColor::Orange => Some(cosmic::iced::Color::from_rgb(1.0, 0.55, 0.0)),
            TextColor::Red => Some(cosmic::iced::Color::from_rgb(0.9, 0.2, 0.2)),
        }
    }
}
