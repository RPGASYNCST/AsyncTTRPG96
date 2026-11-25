use iced::widget::{
    Space, button, column, container, image, pick_list, radio, row, text, text_input,
};
use iced::{Alignment, Element, Length};

use crate::ui::app::RetroApp;
use crate::ui::models::{Message, RETRO_UI_FONT, RpgSystem, Screen, SessionVisibility};
use crate::ui::theme::{
    INPUT_BG, TEXT_DARK, default_button_style, group_box, login_button_style, login_input_style,
    next_pick_list_menu_style, next_pick_list_style, next_radio_style, status_bar_box, sunken_3d,
};
use crate::ui::views::{field_row, status_color};

impl RetroApp {
    pub(crate) fn view_create_session(&self) -> Element<'_, Message> {
        let logo_icon = image(self.logo_handle.clone())
            .width(96)
            .height(96)
            .filter_method(image::FilterMethod::Linear);

        let logo_col = container(logo_icon)
            .padding([12, 6])
            .align_x(Alignment::Center)
            .align_y(Alignment::Center);

        let session_group = group_box(
            "1. Dados da Sessão & Mestre",
            RETRO_UI_FONT,
            column![
                field_row(
                    "Nome da Sessão:",
                    122.0,
                    sunken_3d(
                        text_input("Crônicas de Kharos", &self.session_name)
                            .font(RETRO_UI_FONT)
                            .size(17)
                            .style(login_input_style)
                            .padding([2, 5])
                            .on_input(Message::SessionNameChanged)
                            .on_submit(Message::TabPressed(false)),
                        INPUT_BG,
                        1.5,
                    )
                ),
                field_row(
                    "Opção de RPG:",
                    122.0,
                    sunken_3d(
                        pick_list(
                            RpgSystem::ALL,
                            Some(self.session_system),
                            Message::SessionSystemChanged,
                        )
                        .font(RETRO_UI_FONT)
                        .text_size(17)
                        .width(Length::Fill)
                        .padding([2, 5])
                        .style(next_pick_list_style)
                        .menu_style(next_pick_list_menu_style),
                        INPUT_BG,
                        1.5,
                    )
                ),
                field_row(
                    "Mestre (Host):",
                    122.0,
                    sunken_3d(
                        text_input("Lip1Z", &self.session_host)
                            .font(RETRO_UI_FONT)
                            .size(17)
                            .style(login_input_style)
                            .padding([2, 5])
                            .on_input(Message::SessionHostChanged)
                            .on_submit(Message::TabPressed(false)),
                        INPUT_BG,
                        1.5,
                    )
                ),
                field_row(
                    "E-mail do Mestre:",
                    122.0,
                    sunken_3d(
                        text_input("mestre@chronos.local", &self.session_email)
                            .font(RETRO_UI_FONT)
                            .size(17)
                            .style(login_input_style)
                            .padding([2, 5])
                            .on_input(Message::SessionEmailChanged)
                            .on_submit(Message::TabPressed(false)),
                        INPUT_BG,
                        1.5,
                    )
                ),
                field_row(
                    "Chave do Mestre:",
                    122.0,
                    sunken_3d(
                        text_input("", &self.session_password)
                            .font(RETRO_UI_FONT)
                            .size(17)
                            .style(login_input_style)
                            .secure(true)
                            .padding([2, 5])
                            .on_input(Message::SessionPasswordChanged)
                            .on_submit(Message::TabPressed(false)),
                        INPUT_BG,
                        1.5,
                    )
                ),
            ]
            .spacing(5),
            [12, 10],
        );

        let top_section = row![logo_col, Space::new().width(8), session_group]
            .align_y(Alignment::Center)
            .width(Length::Fill);

        let node_group = if self.session_awaiting_confirmation {
            group_box(
                "2. Confirmação por E-mail do Mestre",
                RETRO_UI_FONT,
                column![
                    text("Um código de 6 dígitos e link foram enviados para o e-mail do Mestre.")
                        .size(16)
                        .font(RETRO_UI_FONT)
                        .color(TEXT_DARK),
                    field_row(
                        "Código (6 dígitos):",
                        155.0,
                        sunken_3d(
                            text_input("000000", &self.session_code_input)
                                .font(RETRO_UI_FONT)
                                .size(17)
                                .style(login_input_style)
                                .padding([2, 5])
                                .on_input(Message::SessionCodeInputChanged)
                                .on_submit(Message::VerifySessionCode),
                            INPUT_BG,
                            1.5,
                        )
                    ),
                ]
                .spacing(6),
                [12, 10],
            )
        } else {
            let vis_radios = row![
                radio(
                    "Privada (Convite)",
                    SessionVisibility::Private,
                    Some(self.session_visibility),
                    Message::SessionVisibilityChanged,
                )
                .size(14)
                .text_size(17)
                .font(RETRO_UI_FONT)
                .style(next_radio_style),
                Space::new().width(20),
                radio(
                    "Pública",
                    SessionVisibility::Public,
                    Some(self.session_visibility),
                    Message::SessionVisibilityChanged,
                )
                .size(14)
                .text_size(17)
                .font(RETRO_UI_FONT)
                .style(next_radio_style),
            ]
            .align_y(Alignment::Center);

            group_box(
                "2. Parâmetros da Mesa",
                RETRO_UI_FONT,
                column![
                    field_row(
                        "Máx. de Jogadores:",
                        155.0,
                        sunken_3d(
                            text_input("5", &self.session_max_players)
                                .font(RETRO_UI_FONT)
                                .size(17)
                                .style(login_input_style)
                                .padding([2, 5])
                                .on_input(Message::SessionMaxPlayersChanged)
                                .on_submit(Message::SubmitCreateSession),
                            INPUT_BG,
                            1.5,
                        )
                    ),
                    row![
                        container(
                            text("Visibilidade da Mesa:")
                                .size(17)
                                .font(RETRO_UI_FONT)
                                .color(TEXT_DARK)
                        )
                        .width(155.0)
                        .align_y(Alignment::Center),
                        vis_radios,
                    ]
                    .spacing(6)
                    .align_y(Alignment::Center),
                ]
                .spacing(5),
                [12, 10],
            )
        };

        let (status_msg, status_err) = &self.session_status;
        let status_bar = status_bar_box(
            text(format!("Status: {}", status_msg))
                .size(16)
                .font(RETRO_UI_FONT)
                .color(status_color(*status_err)),
        );

        let left_btn = if self.session_awaiting_confirmation {
            let mut b = button(
                container(
                    text(if self.session_is_sending {
                        "[ Enviando... ]"
                    } else {
                        "[ Reenviar E-mail ]"
                    })
                    .size(17)
                    .font(RETRO_UI_FONT),
                )
                .align_x(Alignment::Center),
            )
            .style(login_button_style)
            .padding([4, 10]);

            if !self.session_is_sending {
                b = b.on_press(Message::ResendSessionEmail);
            }
            b
        } else {
            button(
                container(text("[ Ajuda ]").size(17).font(RETRO_UI_FONT))
                    .align_x(Alignment::Center),
            )
            .style(login_button_style)
            .padding([4, 10])
            .on_press(Message::SessionHelp)
        };

        let cancel_btn = button(
            container(text("[ Cancelar ]").size(17).font(RETRO_UI_FONT)).align_x(Alignment::Center),
        )
        .style(login_button_style)
        .padding([4, 10])
        .on_press(Message::NavigateTo(Screen::Login));

        let submit_btn = if self.session_confirmed {
            button(
                container(text("[ Ir para Login ↵ ]").size(17).font(RETRO_UI_FONT))
                    .align_x(Alignment::Center),
            )
            .style(default_button_style)
            .padding([4, 12])
            .on_press(Message::NavigateTo(Screen::Login))
        } else if self.session_awaiting_confirmation {
            let mut b = button(
                container(text("[ Validar Código ↵ ]").size(17).font(RETRO_UI_FONT))
                    .align_x(Alignment::Center),
            )
            .style(default_button_style)
            .padding([4, 12]);
            if !self.session_is_sending {
                b = b.on_press(Message::VerifySessionCode);
            }
            b
        } else {
            let mut b = button(
                container(
                    text(if self.session_is_sending {
                        "[ Enviando... ]"
                    } else {
                        "[ Criar Sessão ↵ ]"
                    })
                    .size(17)
                    .font(RETRO_UI_FONT),
                )
                .align_x(Alignment::Center),
            )
            .style(default_button_style)
            .padding([4, 12]);
            if !self.session_is_sending {
                b = b.on_press(Message::SubmitCreateSession);
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
            node_group,
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
