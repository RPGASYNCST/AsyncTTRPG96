use iced::widget::{Space, button, column, container, image, row, text, text_input};
use iced::{Alignment, Element, Length};

use crate::ui::app::RetroApp;
use crate::ui::models::{Message, RETRO_UI_FONT, Screen};
use crate::ui::theme::{
    INPUT_BG, default_button_style, group_box, login_button_style, login_input_style,
    status_bar_box, sunken_3d,
};
use crate::ui::views::{field_row, status_color};

impl RetroApp {
    pub(crate) fn view_create_player(&self) -> Element<'_, Message> {
        let logo_icon = image(self.logo_handle.clone())
            .width(96)
            .height(96)
            .filter_method(image::FilterMethod::Linear);

        let logo_col = container(logo_icon)
            .padding([12, 6])
            .align_x(Alignment::Center)
            .align_y(Alignment::Center);

        let id_group = group_box(
            "1. Identificação do Jogador",
            RETRO_UI_FONT,
            column![
                field_row(
                    "Handle / Usuário:",
                    122.0,
                    sunken_3d(
                        text_input("Kaelen", &self.player_username)
                            .font(RETRO_UI_FONT)
                            .size(17)
                            .style(login_input_style)
                            .padding([2, 5])
                            .on_input(Message::PlayerUsernameChanged)
                            .on_submit(Message::TabPressed(false)),
                        INPUT_BG,
                        1.5,
                    )
                ),
                field_row(
                    "Chave de Acesso:",
                    122.0,
                    sunken_3d(
                        text_input("", &self.player_password)
                            .font(RETRO_UI_FONT)
                            .size(17)
                            .style(login_input_style)
                            .secure(true)
                            .padding([2, 5])
                            .on_input(Message::PlayerPasswordChanged)
                            .on_submit(Message::TabPressed(false)),
                        INPUT_BG,
                        1.5,
                    )
                ),
                field_row(
                    "Confirmar Chave:",
                    122.0,
                    sunken_3d(
                        text_input("", &self.player_confirm_password)
                            .font(RETRO_UI_FONT)
                            .size(17)
                            .style(login_input_style)
                            .secure(true)
                            .padding([2, 5])
                            .on_input(Message::PlayerConfirmPasswordChanged)
                            .on_submit(Message::TabPressed(false)),
                        INPUT_BG,
                        1.5,
                    )
                ),
            ]
            .spacing(5),
            [12, 10],
        );

        let top_section = row![logo_col, Space::new().width(8), id_group]
            .align_y(Alignment::Center)
            .width(Length::Fill);

        let bottom_group = group_box(
            "2. Ingresso na Mesa",
            RETRO_UI_FONT,
            column![field_row(
                "Código de Convite:",
                155.0,
                sunken_3d(
                    text_input("Crônicas de Kharos", &self.player_invite_code)
                        .font(RETRO_UI_FONT)
                        .size(17)
                        .style(login_input_style)
                        .padding([2, 5])
                        .on_input(Message::PlayerInviteCodeChanged)
                        .on_submit(Message::SubmitCreatePlayer),
                    INPUT_BG,
                    1.5,
                )
            ),]
            .spacing(5),
            [12, 10],
        );

        let (status_msg, status_err) = &self.player_status;
        let status_bar = status_bar_box(
            text(format!("Status: {}", status_msg))
                .size(16)
                .font(RETRO_UI_FONT)
                .color(status_color(*status_err)),
        );

        let left_btn = button(
            container(text("[ Ajuda ]").size(17).font(RETRO_UI_FONT)).align_x(Alignment::Center),
        )
        .style(login_button_style)
        .padding([4, 10])
        .on_press(Message::PlayerHelp);

        let cancel_btn = button(
            container(text("[ Cancelar ]").size(17).font(RETRO_UI_FONT)).align_x(Alignment::Center),
        )
        .style(login_button_style)
        .padding([4, 10])
        .on_press(Message::NavigateTo(Screen::Login));

        let submit_btn = if self.player_confirmed {
            button(
                container(text("[ Ir para Login ↵ ]").size(17).font(RETRO_UI_FONT))
                    .align_x(Alignment::Center),
            )
            .style(default_button_style)
            .padding([4, 12])
            .on_press(Message::NavigateTo(Screen::Login))
        } else {
            let mut b = button(
                container(
                    text(if self.player_is_sending {
                        "[ Cadastrando... ]"
                    } else {
                        "[ Criar Jogador ↵ ]"
                    })
                    .size(17)
                    .font(RETRO_UI_FONT),
                )
                .align_x(Alignment::Center),
            )
            .style(default_button_style)
            .padding([4, 12]);
            if !self.player_is_sending {
                b = b.on_press(Message::SubmitCreatePlayer);
            }
            b
        };

        let action_bar = row![
            left_btn,
            Space::new().width(Length::Fill),
            cancel_btn,
            Space::new().width(8),
            submit_btn,
        ]
        .align_y(Alignment::Center)
        .width(Length::Fill);

        container(column![
            top_section,
            Space::new().height(6),
            bottom_group,
            Space::new().height(8),
            status_bar,
            Space::new().height(8),
            action_bar,
        ])
        .padding([8, 10])
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
