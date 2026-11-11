use iced::widget::{Space, button, column, container, image, mouse_area, row, text, text_input};
use iced::{Alignment, Color, Element, Length, Task};

use crate::ui::theme::*;

#[derive(Debug, Clone)]
pub enum Message {
    UsernameChanged(String),
    AccessKeyChanged(String),
    Connect,
    CreateSheet,
    Options,
    DragWindow,
    Minimize,
    Close,
}

pub const RETRO_HEADER_FONT: iced::Font = iced::Font {
    family: iced::font::Family::Name("Silkscreen"),
    weight: iced::font::Weight::Bold,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

pub const RETRO_UI_FONT: iced::Font = iced::Font::with_name("VT323");

pub struct RetroApp {
    username: String,
    access_key: String,
    status_info: Option<String>,
}

impl Default for RetroApp {
    fn default() -> Self {
        Self::new()
    }
}

impl RetroApp {
    pub fn new() -> Self {
        Self {
            username: String::new(),
            access_key: String::new(),
            status_info: None,
        }
    }

    pub fn title(&self) -> String {
        "Sessão de Acesso - RPG ASYNC".to_string()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::UsernameChanged(val) => {
                self.username = val;
            }
            Message::AccessKeyChanged(val) => {
                self.access_key = val;
            }
            Message::Connect => {
                if self.username.trim().is_empty() {
                    self.status_info = Some("Informe o usuário.".to_string());
                } else {
                    self.status_info = Some(format!("Conectando '{}'...", self.username));
                }
            }
            Message::CreateSheet => {
                self.status_info = Some("Criar Sessao selecionado.".to_string());
            }
            Message::Options => {
                self.status_info = Some("Opções selecionado.".to_string());
            }
            Message::DragWindow => {
                return iced::window::oldest().and_then(iced::window::drag);
            }
            Message::Minimize => {
                return iced::window::oldest().and_then(|id| iced::window::minimize(id, true));
            }
            Message::Close => {
                std::process::exit(0);
            }
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let left_menu_btn = button(
            container(text("—").size(16).font(RETRO_UI_FONT))
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center),
        )
        .width(24)
        .height(24)
        .padding(0)
        .style(title_button_style)
        .on_press(Message::Minimize);

        let title_center = mouse_area(
            container(
                text("Sessão de Acesso - RPG ASYNC")
                    .size(18)
                    .font(RETRO_UI_FONT)
                    .color(TEXT_LIGHT),
            )
            .width(Length::Fill)
            .height(24)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .style(title_bar_segment),
        )
        .on_press(Message::DragWindow);

        let close_btn = button(
            container(text("□").size(14).font(RETRO_UI_FONT))
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center),
        )
        .width(24)
        .height(24)
        .padding(0)
        .style(title_button_style)
        .on_press(Message::Close);

        let title_bar = row![left_menu_btn, title_center, close_btn]
            .spacing(1)
            .align_y(Alignment::Center);

        let d20_handle =
            image::Handle::from_bytes(&include_bytes!("../../assets/icons/d20.png")[..]);
        let d20_icon = image(d20_handle)
            .width(50)
            .height(56)
            .filter_method(image::FilterMethod::Nearest);

        let top_header_box = etched_panel(
            row![
                d20_icon,
                Space::new().width(12),
                text("SESSÃO DE ACESSO")
                    .size(20)
                    .font(RETRO_HEADER_FONT)
                    .color(TEXT_DARK),
            ]
            .align_y(Alignment::Center),
            [10, 14],
        );

        let user_input = sunken_3d(
            text_input("", &self.username)
                .font(RETRO_UI_FONT)
                .size(18)
                .style(login_input_style)
                .padding([3, 6])
                .on_input(Message::UsernameChanged),
            INPUT_SUNKEN_BG,
            2.0,
        );

        let key_input = sunken_3d(
            text_input("", &self.access_key)
                .font(RETRO_UI_FONT)
                .size(18)
                .style(login_input_style)
                .secure(true)
                .padding([3, 6])
                .on_input(Message::AccessKeyChanged),
            INPUT_SUNKEN_BG,
            2.0,
        );

        let left_fields = column![
            text("Usuário")
                .size(19)
                .font(RETRO_UI_FONT)
                .color(TEXT_DARK),
            Space::new().height(3),
            user_input,
            Space::new().height(12),
            text("Senha").size(19).font(RETRO_UI_FONT).color(TEXT_DARK),
            Space::new().height(3),
            key_input,
            if let Some(msg) = &self.status_info {
                Element::from(column![
                    Space::new().height(4),
                    text(msg)
                        .size(15)
                        .font(RETRO_UI_FONT)
                        .color(Color::from_rgb(0.55, 0.12, 0.10)),
                ])
            } else {
                Space::new().height(0).into()
            }
        ]
        .width(205);

        let connect_btn_inner = button(
            container(text("Conectar").size(18).font(RETRO_UI_FONT))
                .width(Length::Fill)
                .align_x(Alignment::Center),
        )
        .style(login_button_style)
        .width(Length::Fill)
        .padding([4, 8])
        .on_press(Message::Connect);

        let connect_btn = container(sunken_3d(
            container(connect_btn_inner).padding(2),
            CDE_GRAY,
            1.5,
        ))
        .width(114);

        let sheet_btn = container(
            button(
                container(text("Criar sessão").size(18).font(RETRO_UI_FONT))
                    .width(Length::Fill)
                    .align_x(Alignment::Center),
            )
            .style(login_button_style)
            .width(Length::Fill)
            .padding([4, 8])
            .on_press(Message::CreateSheet),
        )
        .width(106);

        let options_btn = container(
            button(
                container(text("Opções").size(18).font(RETRO_UI_FONT))
                    .width(Length::Fill)
                    .align_x(Alignment::Center),
            )
            .style(login_button_style)
            .width(Length::Fill)
            .padding([4, 8])
            .on_press(Message::Options),
        )
        .width(106);

        let right_buttons = column![
            connect_btn,
            Space::new().height(8),
            sheet_btn,
            Space::new().height(8),
            options_btn,
        ]
        .align_x(Alignment::Center);

        let bottom_form_box = etched_panel(
            row![left_fields, Space::new().width(22), right_buttons].align_y(Alignment::Center),
            [14, 16],
        );

        let inner_body = container(column![
            top_header_box,
            Space::new().height(4),
            bottom_form_box,
        ])
        .padding(6)
        .width(Length::Fill)
        .height(Length::Fill);

        container(column![title_bar, inner_body])
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

    iced::application(RetroApp::new, RetroApp::update, RetroApp::view)
        .title(RetroApp::title)
        .font(include_bytes!("../../assets/fonts/Silkscreen-Regular.ttf").as_slice())
        .font(include_bytes!("../../assets/fonts/Silkscreen-Bold.ttf").as_slice())
        .font(include_bytes!("../../assets/fonts/VT323-Regular.ttf").as_slice())
        .default_font(RETRO_UI_FONT)
        .window(iced::window::Settings {
            size: iced::Size::new(396.0, 290.0),
            position: iced::window::Position::Centered,
            resizable: false,
            decorations: false,
            icon: app_icon,
            ..Default::default()
        })
        .run()
}
