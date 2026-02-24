mod config;

use config::{Config, PublicIpService, TextColor};
use cosmic::app::Core;
use cosmic::iced::{
    platform_specific::shell::commands::popup::{destroy_popup, get_popup},
    window::Id,
    Color, Length, Subscription,
};
use cosmic::{Action, Element, Task};
use std::collections::HashMap;
use std::time::Duration;

const APP_ID: &str = "io.github.Pontus-Ottosson.CosmicIpApplet";
const REFRESH_RATES: &[u64] = &[5, 10, 15, 30, 60];

#[derive(Debug, Clone)]
pub enum Message {
    TogglePopup,
    PopupClosed(Id),
    NetworkIpsUpdated(HashMap<String, String>),
    PublicIpUpdated(String),
    Tick,
    ToggleInterface(String),
    TogglePublicIp(bool),
    SetTextColor(TextColor),
    SetPublicIpService(PublicIpService),
    SetRefreshRate(u64),
    SetActiveTab(SettingsTab),
    CopyToClipboard(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettingsTab { Info, Settings }

pub struct IpApplet {
    core: Core,
    popup: Option<Id>,
    main_window_id: Id,
    network_ips: HashMap<String, String>,
    known_interfaces: Vec<String>,
    public_ip: String,
    config: Config,
    active_tab: SettingsTab,
}

impl cosmic::Application for IpApplet {
    type Executor = cosmic::SingleThreadExecutor;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = APP_ID;

    fn core(&self) -> &Core { &self.core }
    fn core_mut(&mut self) -> &mut Core { &mut self.core }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Action<Self::Message>>) {
        let main_window_id = core.main_window_id().unwrap_or(Id::RESERVED);
        let url = Config::default().public_ip_service.url().to_string();
        let app = Self {
            core, popup: None, main_window_id,
            network_ips: HashMap::new(),
            known_interfaces: Vec::new(),
            public_ip: "Fetching...".to_string(),
            config: Config::default(),
            active_tab: SettingsTab::Info,
        };
        let task = Task::batch(vec![
            Task::perform(fetch_network_ips(), |ips| Action::App(Message::NetworkIpsUpdated(ips))),
            Task::perform(fetch_public_ip(url), |ip| Action::App(Message::PublicIpUpdated(ip))),
        ]);
        (app, task)
    }

    fn on_close_requested(&self, id: Id) -> Option<Message> { Some(Message::PopupClosed(id)) }

    fn update(&mut self, msg: Self::Message) -> Task<Action<Self::Message>> {
        match msg {
            Message::TogglePopup => {
                return if let Some(p) = self.popup.take() {
                    destroy_popup(p)
                } else {
                    let new_id = Id::unique();
                    self.popup = Some(new_id);
                    let mut s = self.core.applet.get_popup_settings(
                        self.main_window_id, new_id, None, None, None,
                    );
                    s.positioner.size_limits = cosmic::iced::Limits::NONE.min_width(260.0).max_width(360.0);
                    get_popup(s)
                };
            }
            Message::PopupClosed(id) => { if self.popup == Some(id) { self.popup = None; } }
            Message::NetworkIpsUpdated(ips) => {
                // Sync known_interfaces: keep stable sort, add new ones, remove gone ones
                let mut new_ifaces: Vec<String> = ips.keys().cloned().collect();
                new_ifaces.sort();
                // Auto-enable any interface we haven't seen before
                for iface in &new_ifaces {
                    if !self.known_interfaces.contains(iface) {
                        self.config.enabled_interfaces.insert(iface.clone());
                    }
                }
                // Remove interfaces that no longer exist from enabled set
                self.known_interfaces.retain(|i| new_ifaces.contains(i));
                for iface in &new_ifaces {
                    if !self.known_interfaces.contains(iface) {
                        self.known_interfaces.push(iface.clone());
                    }
                }
                self.network_ips = ips;
            }
            Message::PublicIpUpdated(ip) => { self.public_ip = ip; }
            Message::Tick => {
                let url = self.config.public_ip_service.url().to_string();
                return Task::batch(vec![
                    Task::perform(fetch_network_ips(), |ips| Action::App(Message::NetworkIpsUpdated(ips))),
                    Task::perform(fetch_public_ip(url), |ip| Action::App(Message::PublicIpUpdated(ip))),
                ]);
            }
            Message::ToggleInterface(iface) => {
                if self.config.enabled_interfaces.contains(&iface) {
                    self.config.enabled_interfaces.remove(&iface);
                } else {
                    self.config.enabled_interfaces.insert(iface);
                }
            }
            Message::TogglePublicIp(val) => { self.config.show_public_ip = val; }
            Message::SetTextColor(c) => { self.config.text_color = c; }
            Message::SetPublicIpService(s) => {
                let url = s.url().to_string();
                self.config.public_ip_service = s;
                return Task::perform(fetch_public_ip(url), |ip| Action::App(Message::PublicIpUpdated(ip)));
            }
            Message::SetRefreshRate(r) => { self.config.refresh_rate_secs = r; }
            Message::SetActiveTab(t) => { self.active_tab = t; }
            Message::CopyToClipboard(text) => {
                return cosmic::iced::clipboard::write::<Action<Message>>(text);
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        self.core.applet.icon_button("globe-alt2-symbolic")
            .on_press(Message::TogglePopup)
            .into()
    }

    fn view_window(&self, _id: Id) -> Element<'_, Self::Message> {
        let info_sel = self.active_tab == SettingsTab::Info;
        let tab_bar = cosmic::widget::row::with_children(vec![
            cosmic::widget::button::text(if info_sel { "* IP addresses" } else { "  IP addresses" })
                .on_press(Message::SetActiveTab(SettingsTab::Info))
                .class(if info_sel { cosmic::theme::Button::Suggested } else { cosmic::theme::Button::Text })
                .into(),
            cosmic::widget::button::text(if !info_sel { "* Settings" } else { "  Settings" })
                .on_press(Message::SetActiveTab(SettingsTab::Settings))
                .class(if !info_sel { cosmic::theme::Button::Suggested } else { cosmic::theme::Button::Text })
                .into(),
        ]).spacing(4).padding([8, 12]);

        let content: Element<'_, Message> = match self.active_tab {
            SettingsTab::Info => self.view_info_tab(),
            SettingsTab::Settings => self.view_settings_tab(),
        };

        let popup_content = cosmic::widget::column::with_children(vec![
            tab_bar.into(),
            cosmic::widget::divider::horizontal::default().into(),
            content,
        ]).padding([0, 0, 8, 0]);

        self.core.applet.popup_container(popup_content).into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        cosmic::iced::time::every(Duration::from_secs(self.config.refresh_rate_secs))
            .map(|_| Message::Tick)
    }
}

impl IpApplet {
    fn view_info_tab(&self) -> Element<'_, Message> {
        let uc = self.config.text_color.to_iced_color();
        let mut col = cosmic::widget::column::with_capacity(8).spacing(8).padding([12, 16]);
        let mut has_any = false;

        for iface in &self.known_interfaces {
            if self.config.enabled_interfaces.contains(iface.as_str()) {
                if let Some(ip) = self.network_ips.get(iface) {
                    has_any = true;
                    let label = cosmic::widget::text(iface.as_str()).size(11)
                        .class(cosmic::theme::Text::Color(Color::from_rgb(0.55, 0.55, 0.55)));
                    let val = match uc {
                        Some(c) => cosmic::widget::text(ip.as_str()).size(15).class(cosmic::theme::Text::Color(c)),
                        None    => cosmic::widget::text(ip.as_str()).size(15),
                    };
                    let ip_clone = ip.clone();
                    let copy_btn = self.core.applet.icon_button("edit-copy-symbolic")
                        .on_press(Message::CopyToClipboard(ip_clone));
                    let ip_row = cosmic::widget::row::with_children(vec![
                        val.into(),
                        copy_btn.into(),
                    ])
                    .align_y(cosmic::iced::Alignment::Center)
                    .spacing(4);
                    col = col.push(
                        cosmic::widget::column::with_children(vec![label.into(), ip_row.into()]).spacing(1)
                    );
                }
            }
        }

        if self.config.show_public_ip {
            has_any = true;
            let label = cosmic::widget::text("Public IP").size(11)
                .class(cosmic::theme::Text::Color(Color::from_rgb(0.55, 0.55, 0.55)));
            let val = match uc {
                Some(c) => cosmic::widget::text(self.public_ip.as_str()).size(15).class(cosmic::theme::Text::Color(c)),
                None    => cosmic::widget::text(self.public_ip.as_str()).size(15),
            };
            let copy_btn = self.core.applet.icon_button("edit-copy-symbolic")
                .on_press(Message::CopyToClipboard(self.public_ip.clone()));
            let ip_row = cosmic::widget::row::with_children(vec![
                val.into(),
                copy_btn.into(),
            ])
            .align_y(cosmic::iced::Alignment::Center)
            .spacing(4);
            col = col.push(
                cosmic::widget::column::with_children(vec![label.into(), ip_row.into()]).spacing(1)
            );
        }

        if !has_any {
            col = col.push(cosmic::widget::text("No active interfaces").size(12));
        }
        col.into()
    }

    fn view_settings_tab(&self) -> Element<'_, Message> {
        let muted = cosmic::theme::Text::Color(Color::from_rgb(0.55, 0.55, 0.55));
        let mut col = cosmic::widget::column::with_capacity(24).spacing(4).padding([10, 16]);

        col = col.push(cosmic::widget::text("SHOW INTERFACES").size(11).class(muted.clone()));
        for iface in &self.known_interfaces {
            let name = iface.clone();
            let enabled = self.config.enabled_interfaces.contains(&name);
            let n = name.clone();
            col = col.push(
                cosmic::widget::toggler(enabled)
                    .label(name)
                    .on_toggle(move |_| Message::ToggleInterface(n.clone()))
            );
        }

        col = col.push(cosmic::widget::divider::horizontal::default());
        col = col.push(cosmic::widget::text("PUBLIC IP").size(11).class(muted.clone()));
        col = col.push(
            cosmic::widget::toggler(self.config.show_public_ip)
                .label("Show Public IP")
                .on_toggle(Message::TogglePublicIp)
        );

        if self.config.show_public_ip {
            col = col.push(cosmic::widget::text("SERVICE").size(11).class(muted.clone()));
            for service in PublicIpService::all() {
                let sel = self.config.public_ip_service == service;
                let lbl = format!("{}{}", if sel { "* " } else { "  " }, service.label());
                let s = service.clone();
                col = col.push(
                    cosmic::widget::button::text(lbl).on_press(Message::SetPublicIpService(s))
                        .class(if sel { cosmic::theme::Button::Suggested } else { cosmic::theme::Button::Text })
                );
            }
        }

        col = col.push(cosmic::widget::divider::horizontal::default());
        col = col.push(cosmic::widget::text("REFRESH RATE").size(11).class(muted.clone()));
        let mut rr = cosmic::widget::row::with_capacity(5).spacing(4);
        for &rate in REFRESH_RATES {
            let sel = self.config.refresh_rate_secs == rate;
            rr = rr.push(
                cosmic::widget::button::text(format!("{}s", rate)).on_press(Message::SetRefreshRate(rate))
                    .class(if sel { cosmic::theme::Button::Suggested } else { cosmic::theme::Button::Standard })
            );
        }
        col = col.push(rr);

        col = col.push(cosmic::widget::divider::horizontal::default());
        col = col.push(cosmic::widget::text("TEXT COLOR").size(11).class(muted.clone()));
        for tc in TextColor::all() {
            let sel = self.config.text_color == tc;
            let lbl = format!("{}{}", if sel { "* " } else { "  " }, tc.label());
            let c = tc.clone();
            col = col.push(
                cosmic::widget::button::text(lbl).on_press(Message::SetTextColor(c))
                    .class(if sel { cosmic::theme::Button::Suggested } else { cosmic::theme::Button::Text })
            );
        }

        cosmic::widget::scrollable(col).height(Length::Fixed(400.0)).into()
    }
}

async fn fetch_network_ips() -> HashMap<String, String> {
    let mut result = HashMap::new();
    if let Ok(ifaces) = if_addrs::get_if_addrs() {
        for iface in ifaces {
            // Skip loopback and any interface without an IPv4 address
            if iface.is_loopback() { continue; }
            if let if_addrs::IfAddr::V4(v4) = iface.addr {
                result.insert(iface.name, v4.ip.to_string());
            }
        }
    }
    result
}

async fn fetch_public_ip(url: String) -> String {
    match reqwest::get(&url).await {
        Ok(resp) => resp.text().await.unwrap_or_default().trim().to_string(),
        Err(_) => "Not available".to_string(),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cosmic::applet::run::<IpApplet>(())?;
    Ok(())
}
