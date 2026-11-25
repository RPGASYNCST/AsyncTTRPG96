use iced::widget::{Space, button, column, container, image, row, text, text_input};
use iced::{Alignment, Element, Length};

use crate::ui::app::RetroApp;
use crate::ui::models::{Message, RETRO_HEADER_FONT, RETRO_UI_FONT, Screen};
use crate::ui::theme::{
    INPUT_BG, STATUS_ERROR, STATUS_SUCCESS, TEXT_DARK, default_button_style, etched_panel,
    login_button_style, login_input_style, sunken_3d,
};

impl RetroApp {
    pub(crate) fn view_login(&self) -> Element<'_, Message> {
        let d20_icon = image(self.d20_handle.clone())
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
                .on_input(Message::UsernameChanged)
                .on_submit(Message::TabPressed(false)),
            INPUT_BG,
            1.5,
        );

        let key_input = sunken_3d(
            text_input("", &self.access_key)
                .font(RETRO_UI_FONT)
                .size(18)
                .style(login_input_style)
                .secure(true)
                .padding([3, 6])
                .on_input(Message::AccessKeyChanged)
                .on_submit(Message::Connect),
            INPUT_BG,
            1.5,
        );

        let left_fields = column![
            text("Usuário")
                .size(19)
                .font(RETRO_UI_FONT)
                .color(TEXT_DARK),
            Space::new().height(3),
            user_input,
            Space::new().height(10),
            text("Senha").size(19).font(RETRO_UI_FONT).color(TEXT_DARK),
            Space::new().height(3),
            key_input,
        ]
        .width(205);

        let connect_btn = button(
            container(text("Conectar").size(18).font(RETRO_UI_FONT))
                .width(Length::Fill)
                .align_x(Alignment::Center),
        )
        .style(default_button_style)
        .width(114)
        .padding([4, 8])
        .on_press(Message::Connect);

        let sheet_btn = button(
            container(text("Criar sessão").size(18).font(RETRO_UI_FONT))
                .width(Length::Fill)
                .align_x(Alignment::Center),
        )
        .style(login_button_style)
        .width(114)
        .padding([4, 8])
        .on_press(Message::NavigateTo(Screen::CreateSession));

        let player_btn = button(
            container(text("Criar jogador").size(18).font(RETRO_UI_FONT))
                .width(Length::Fill)
                .align_x(Alignment::Center),
        )
        .style(login_button_style)
        .width(114)
        .padding([4, 8])
        .on_press(Message::NavigateTo(Screen::CreatePlayer));

        let options_btn = button(
            container(text("Opções").size(18).font(RETRO_UI_FONT))
                .width(Length::Fill)
                .align_x(Alignment::Center),
        )
        .style(login_button_style)
        .width(114)
        .padding([4, 8])
        .on_press(Message::Options);

        let right_buttons = column![
            connect_btn,
            Space::new().height(6),
            sheet_btn,
            Space::new().height(6),
            player_btn,
            Space::new().height(6),
            options_btn,
        ]
        .align_x(Alignment::Center);

        let status_slot = container(if let Some(msg) = &self.status_info {
            let color = if self.status_is_error {
                STATUS_ERROR
            } else {
                STATUS_SUCCESS
            };
            Element::from(text(msg).size(15).font(RETRO_UI_FONT).color(color))
        } else {
            Space::new().height(0).into()
        })
        .width(Length::Fill)
        .height(34)
        .align_y(Alignment::Center);

        let bottom_form_box = etched_panel(
            column![
                row![left_fields, Space::new().width(22), right_buttons].align_y(Alignment::Center),
                Space::new().height(6),
                status_slot,
            ],
            [10, 16],
        );

        container(column![
            top_header_box,
            Space::new().height(4),
            bottom_form_box,
        ])
        .padding(6)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
