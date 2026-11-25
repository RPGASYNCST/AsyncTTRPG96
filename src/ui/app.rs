use iced::widget::{button, column, container, image, mouse_area, row, text, text_editor};
use iced::{Alignment, Element, Event, Length, Subscription, keyboard};

use crate::ui::models::{
    DndAlignment, DndClass, DndRace, Message, RETRO_UI_FONT, RpgSystem, Screen, SessionVisibility,
};
use crate::ui::sprite::{SpriteConfig, render_sprite_png};
use crate::ui::theme::{TEXT_LIGHT, outer_window_frame, title_bar_segment, title_button_style};

pub struct RetroApp {
    pub(crate) screen: Screen,

    pub(crate) d20_handle: image::Handle,
    pub(crate) logo_handle: image::Handle,
    pub(crate) portrait_handle: image::Handle,

    pub(crate) active_sessao_id: u64,
    pub(crate) active_sessao_nome: String,
    pub(crate) active_sessao_sistema: String,
    pub(crate) active_players_id: u64,

    pub(crate) username: String,
    pub(crate) access_key: String,
    pub(crate) status_info: Option<String>,
    pub(crate) status_is_error: bool,
    pub(crate) login_in_progress: bool,
    pub(crate) auth_token: Option<String>,

    pub(crate) session_name: String,
    pub(crate) session_system: RpgSystem,
    pub(crate) session_host: String,
    pub(crate) session_email: String,
    pub(crate) session_password: String,
    pub(crate) session_max_players: String,
    pub(crate) session_visibility: SessionVisibility,
    pub(crate) session_status: (String, Option<bool>),
    pub(crate) session_is_sending: bool,
    pub(crate) session_awaiting_confirmation: bool,
    pub(crate) session_expected_code: Option<String>,
    pub(crate) session_verification_token: Option<String>,
    pub(crate) session_code_input: String,
    pub(crate) session_confirmed: bool,

    pub(crate) player_username: String,
    pub(crate) player_password: String,
    pub(crate) player_confirm_password: String,
    pub(crate) player_invite_code: String,
    pub(crate) player_status: (String, Option<bool>),
    pub(crate) player_is_sending: bool,
    pub(crate) player_confirmed: bool,

    pub(crate) sheet_name: String,
    pub(crate) sheet_race: DndRace,
    pub(crate) sheet_class: DndClass,
    pub(crate) sheet_alignment: DndAlignment,
    pub(crate) sheet_items: String,
    pub(crate) sheet_gold: String,
    pub(crate) sheet_bio: text_editor::Content,
    pub(crate) sheet_attr_for: String,
    pub(crate) sheet_attr_des: String,
    pub(crate) sheet_attr_con: String,
    pub(crate) sheet_attr_int: String,
    pub(crate) sheet_attr_sab: String,
    pub(crate) sheet_attr_car: String,
    pub(crate) sheet_portrait_filename: Option<String>,
    pub(crate) sheet_portrait_base64: Option<String>,
    pub(crate) sheet_status: (String, Option<bool>),
    pub(crate) sheet_is_saving: bool,
    pub(crate) sheet_saved: bool,

    pub(crate) sprite_config: SpriteConfig,
    pub(crate) sprite_preview_handle: image::Handle,
    pub(crate) sprite_preview_b64: String,
    pub(crate) sprite_status: String,
}

impl Default for RetroApp {
    fn default() -> Self {
        Self::new()
    }
}

impl RetroApp {
    pub fn new() -> Self {
        let default_sprite = SpriteConfig::default();
        let (_, _, sprite_png, sprite_b64) = render_sprite_png(&default_sprite);
        Self {
            screen: Screen::Login,
            d20_handle: image::Handle::from_bytes(
                &include_bytes!("../../assets/icons/d20.png")[..],
            ),
            logo_handle: image::Handle::from_bytes(
                &include_bytes!("../../assets/icons/logo.png")[..],
            ),
            portrait_handle: image::Handle::from_bytes(
                &include_bytes!("../../assets/icons/portrait_default.png")[..],
            ),
            active_sessao_id: 1,
            active_sessao_nome: "Crônicas de Kharos".to_string(),
            active_sessao_sistema: "D&D".to_string(),
            active_players_id: 1,
            username: String::new(),
            access_key: String::new(),
            status_info: None,
            status_is_error: true,
            login_in_progress: false,
            auth_token: None,
            session_name: String::new(),
            session_system: RpgSystem::DnD,
            session_host: String::new(),
            session_email: String::new(),
            session_password: String::new(),
            session_max_players: "5".to_string(),
            session_visibility: SessionVisibility::Private,
            session_status: ("Aguardando configuração da nova sessão.".to_string(), None),
            session_is_sending: false,
            session_awaiting_confirmation: false,
            session_expected_code: None,
            session_verification_token: None,
            session_code_input: String::new(),
            session_confirmed: false,
            player_username: String::new(),
            player_password: String::new(),
            player_confirm_password: String::new(),
            player_invite_code: String::new(),
            player_status: (
                "Aguardando preenchimento das credenciais do jogador.".to_string(),
                None,
            ),
            player_is_sending: false,
            player_confirmed: false,
            sheet_name: "Eldrin Sombra".to_string(),
            sheet_race: DndRace::Elfo,
            sheet_class: DndClass::Ladino,
            sheet_alignment: DndAlignment::CaoticoNeutro,
            sheet_items: "Espada Curta, Armadura de Couro, Gazetas, Tochas, Ração".to_string(),
            sheet_gold: "120".to_string(),
            sheet_bio: text_editor::Content::with_text(
                "Órfão das ruas de Kharos, sobrevivente dos becos de engrenagens. \
Especialista em obter o que não lhe pertence e caçar segredos da fenda temporal...",
            ),
            sheet_attr_for: "14".to_string(),
            sheet_attr_des: "16".to_string(),
            sheet_attr_con: "12".to_string(),
            sheet_attr_int: "12".to_string(),
            sheet_attr_sab: "10".to_string(),
            sheet_attr_car: "11".to_string(),
            sheet_portrait_filename: None,
            sheet_portrait_base64: None,
            sheet_status: (
                "Pronto para salvar e submeter a ficha para a mesa.".to_string(),
                None,
            ),
            sheet_is_saving: false,
            sheet_saved: false,
            sprite_config: default_sprite,
            sprite_preview_handle: image::Handle::from_bytes(sprite_png),
            sprite_preview_b64: sprite_b64,
            sprite_status:
                "Personalize as camadas 16-bit do seu personagem e clique em [ Aplicar Sprite ↵ ]."
                    .to_string(),
        }
    }

    pub(crate) fn refresh_sprite_preview(&mut self) {
        let (_, _, png_bytes, b64) = render_sprite_png(&self.sprite_config);
        self.sprite_preview_handle = image::Handle::from_bytes(png_bytes);
        self.sprite_preview_b64 = b64;
    }

    pub fn title(&self) -> String {
        match self.screen {
            Screen::Login => "Sessão de Acesso - RPG ASYNC".to_string(),
            Screen::CreateSession => "RPGASYNCST — Criar Sessão".to_string(),
            Screen::CreatePlayer => "RPGASYNCST — Criar Jogador".to_string(),
            Screen::CreateSheet => {
                format!("RPGASYNCST - Criar Ficha: {}", self.active_sessao_nome)
            }
            Screen::SpriteEditor => {
                format!(
                    "RPGASYNCST - Editor de Sprite 16-Bit: {}",
                    self.sheet_name.trim()
                )
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        iced::event::listen_with(|event, _status, _window| match event {
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(keyboard::key::Named::Tab),
                modifiers,
                ..
            }) => Some(Message::TabPressed(modifiers.shift())),
            _ => None,
        })
    }

    pub fn view(&self) -> Element<'_, Message> {
        let (min_label, title_text, close_label) = match self.screen {
            Screen::Login => ("—", "Sessão de Acesso - RPG ASYNC".to_string(), "✕"),
            Screen::CreateSession => ("[-]", "RPGASYNCST — Criar Sessão".to_string(), "✕"),
            Screen::CreatePlayer => ("[-]", "RPGASYNCST — Criar Jogador".to_string(), "✕"),
            Screen::CreateSheet => (
                "[-]",
                format!("RPGASYNCST - Criar Ficha: {}", self.active_sessao_nome),
                "✕",
            ),
            Screen::SpriteEditor => (
                "[-]",
                format!(
                    "RPGASYNCST - Editor de Sprite 16-Bit: {}",
                    self.sheet_name.trim()
                ),
                "✕",
            ),
        };

        let minimize_btn = button(
            container(text(min_label).size(15).font(RETRO_UI_FONT))
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center),
        )
        .width(24)
        .height(22)
        .padding(0)
        .style(title_button_style)
        .on_press(Message::Minimize);

        let title_center = mouse_area(
            container(
                text(title_text)
                    .size(18)
                    .font(RETRO_UI_FONT)
                    .color(TEXT_LIGHT),
            )
            .width(Length::Fill)
            .height(22)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .style(title_bar_segment),
        )
        .on_press(Message::DragWindow);

        let close_btn = button(
            container(text(close_label).size(14).font(RETRO_UI_FONT))
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center),
        )
        .width(24)
        .height(22)
        .padding(0)
        .style(title_button_style)
        .on_press(Message::Close);

        let title_bar = row![minimize_btn, title_center, close_btn]
            .spacing(1)
            .align_y(Alignment::Center);

        let content = match self.screen {
            Screen::Login => self.view_login(),
            Screen::CreateSession => self.view_create_session(),
            Screen::CreatePlayer => self.view_create_player(),
            Screen::CreateSheet => self.view_create_sheet(),
            Screen::SpriteEditor => self.view_sprite_editor(),
        };

        container(column![title_bar, content])
            .width(Length::Fill)
            .height(Length::Fill)
            .style(outer_window_frame)
            .into()
    }
}

pub fn run_gui() -> iced::Result {
    let app_icon =
        iced::window::icon::from_file_data(include_bytes!("../../assets/icons/logo.png"), None)
            .ok();

    iced::application(
        || {
            (
                RetroApp::new(),
                iced::window::oldest()
                    .and_then(|id| iced::window::resize(id, iced::Size::new(396.0, 326.0))),
            )
        },
        RetroApp::update,
        RetroApp::view,
    )
    .subscription(RetroApp::subscription)
    .title(RetroApp::title)
    .font(include_bytes!("../../assets/fonts/Silkscreen-Regular.ttf").as_slice())
    .font(include_bytes!("../../assets/fonts/Silkscreen-Bold.ttf").as_slice())
    .font(include_bytes!("../../assets/fonts/VT323-Regular.ttf").as_slice())
    .default_font(RETRO_UI_FONT)
    .window(iced::window::Settings {
        size: iced::Size::new(396.0, 326.0),
        position: iced::window::Position::Centered,
        resizable: false,
        decorations: false,
        icon: app_icon,
        ..Default::default()
    })
    .run()
}
