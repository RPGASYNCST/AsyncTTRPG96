use iced::widget::{Space, button, column, container, image, row, text, text_editor, text_input};
use iced::{Alignment, Element, Length};

use crate::ui::app::RetroApp;
use crate::ui::models::{DndAlignment, DndClass, DndRace, Message, RETRO_UI_FONT, Screen};
use crate::ui::theme::{
    INPUT_BG, TEXT_DARK, default_button_style, etched_panel, horizontal_groove, login_button_style,
    login_input_style, next_text_editor_style, sheet_section_panel, status_bar_box, sunken_3d,
};
use crate::ui::views::{attr_row, field_row, retro_pick_list, status_color};

impl RetroApp {
    pub(crate) fn view_create_sheet(&self) -> Element<'_, Message> {
        let portrait_img = image(self.portrait_handle.clone())
            .width(108)
            .height(118)
            .filter_method(image::FilterMethod::Nearest);

        let display_char_name = if self.sheet_name.trim().is_empty() {
            "Sem Nome"
        } else {
            self.sheet_name.trim()
        };

        let gen_ia_btn = button(
            container(text("[ Customizar ↵ ]").size(15).font(RETRO_UI_FONT))
                .width(Length::Fill)
                .align_x(Alignment::Center),
        )
        .style(login_button_style)
        .width(126)
        .padding([2, 6])
        .on_press(Message::SheetOpenSpriteEditor);

        let load_portrait_btn = button(
            container(text("[ Carregar ]").size(15).font(RETRO_UI_FONT))
                .width(Length::Fill)
                .align_x(Alignment::Center),
        )
        .style(login_button_style)
        .width(104)
        .padding([2, 6])
        .on_press(Message::SheetLoadPortrait);

        let retrato_panel = sheet_section_panel(
            "Retrato",
            RETRO_UI_FONT,
            column![
                container(portrait_img)
                    .width(Length::Fill)
                    .align_x(Alignment::Center),
                Space::new().height(2),
                container(
                    text(display_char_name)
                        .size(19)
                        .font(RETRO_UI_FONT)
                        .color(TEXT_DARK)
                )
                .width(Length::Fill)
                .align_x(Alignment::Center),
                Space::new().height(3),
                container(gen_ia_btn)
                    .width(Length::Fill)
                    .align_x(Alignment::Center),
                Space::new().height(3),
                container(load_portrait_btn)
                    .width(Length::Fill)
                    .align_x(Alignment::Center),
            ]
            .align_x(Alignment::Center),
            [6, 8],
        );

        let roll_btn = button(
            container(text("[ Rolar 4d6d1 🎲 ]").size(15).font(RETRO_UI_FONT))
                .width(Length::Fill)
                .align_x(Alignment::Center),
        )
        .style(login_button_style)
        .width(Length::Fill)
        .padding([3, 6])
        .on_press(Message::SheetRollAttributes);

        let atributos_panel = sheet_section_panel(
            "Atributos",
            RETRO_UI_FONT,
            column![
                attr_row(
                    "FOR (Força):",
                    &self.sheet_attr_for,
                    Message::SheetAttrForChanged
                ),
                attr_row(
                    "DES (Destreza):",
                    &self.sheet_attr_des,
                    Message::SheetAttrDesChanged
                ),
                attr_row(
                    "CON (Constituição):",
                    &self.sheet_attr_con,
                    Message::SheetAttrConChanged
                ),
                attr_row(
                    "INT (Inteligência):",
                    &self.sheet_attr_int,
                    Message::SheetAttrIntChanged
                ),
                attr_row(
                    "SAB (Sabedoria):",
                    &self.sheet_attr_sab,
                    Message::SheetAttrSabChanged
                ),
                attr_row(
                    "CAR (Carisma):",
                    &self.sheet_attr_car,
                    Message::SheetAttrCarChanged
                ),
                Space::new().height(4),
                roll_btn,
            ]
            .spacing(3),
            [6, 8],
        );

        let left_col = column![retrato_panel, Space::new().height(6), atributos_panel].width(166);

        let panel_identidade = etched_panel(
            column![
                container(
                    text("Novo Personagem - Registro de Ficha")
                        .size(20)
                        .font(RETRO_UI_FONT)
                        .color(TEXT_DARK)
                )
                .padding([4, 8]),
                horizontal_groove(),
                container(
                    column![
                        text("1. Identidade e Classe")
                            .size(18)
                            .font(RETRO_UI_FONT)
                            .color(TEXT_DARK),
                        Space::new().height(2),
                        field_row(
                            "Nome do Personagem:",
                            138.0,
                            sunken_3d(
                                text_input("Eldrin Sombra", &self.sheet_name)
                                    .font(RETRO_UI_FONT)
                                    .size(17)
                                    .style(login_input_style)
                                    .padding([2, 5])
                                    .on_input(Message::SheetNameChanged)
                                    .on_submit(Message::TabPressed(false)),
                                INPUT_BG,
                                1.5,
                            )
                        ),
                        field_row(
                            "Raça / Origem:",
                            138.0,
                            retro_pick_list(
                                DndRace::ALL,
                                Some(self.sheet_race),
                                Message::SheetRaceChanged,
                                17,
                            )
                        ),
                        field_row(
                            "Classe / Perfil:",
                            138.0,
                            retro_pick_list(
                                DndClass::ALL,
                                Some(self.sheet_class),
                                Message::SheetClassChanged,
                                17,
                            )
                        ),
                        field_row(
                            "Alinhamento:",
                            138.0,
                            retro_pick_list(
                                DndAlignment::ALL,
                                Some(self.sheet_alignment),
                                Message::SheetAlignmentChanged,
                                17,
                            )
                        ),
                    ]
                    .spacing(4)
                )
                .padding([6, 8])
                .width(Length::Fill),
            ]
            .width(Length::Fill),
            0,
        );

        let panel_equipamento = sheet_section_panel(
            "2. Equipamento & Recursos",
            RETRO_UI_FONT,
            column![
                field_row(
                    "Itens Iniciais:",
                    138.0,
                    sunken_3d(
                        text_input(
                            "Espada Curta, Armadura de Couro, Gazetas, Tochas, Ração",
                            &self.sheet_items
                        )
                        .font(RETRO_UI_FONT)
                        .size(16)
                        .style(login_input_style)
                        .padding([3, 5])
                        .on_input(Message::SheetItemsChanged)
                        .on_submit(Message::TabPressed(false)),
                        INPUT_BG,
                        1.5,
                    )
                ),
                row![
                    container(
                        text("Moedas de Ouro (PO):")
                            .size(17)
                            .font(RETRO_UI_FONT)
                            .color(TEXT_DARK)
                    )
                    .width(138.0)
                    .align_y(Alignment::Center),
                    container(sunken_3d(
                        text_input("120", &self.sheet_gold)
                            .font(RETRO_UI_FONT)
                            .size(17)
                            .style(login_input_style)
                            .padding([2, 5])
                            .on_input(Message::SheetGoldChanged)
                            .on_submit(Message::TabPressed(false)),
                        INPUT_BG,
                        1.5,
                    ))
                    .width(72),
                ]
                .spacing(6)
                .align_y(Alignment::Center),
            ]
            .spacing(5),
            [6, 8],
        );

        let help_btn = button(
            container(text("[ Ajuda ]").size(16).font(RETRO_UI_FONT)).align_x(Alignment::Center),
        )
        .style(login_button_style)
        .padding([3, 8])
        .on_press(Message::SheetHelp);

        let cancel_btn = button(
            container(text("[ Cancelar ]").size(16).font(RETRO_UI_FONT)).align_x(Alignment::Center),
        )
        .style(login_button_style)
        .padding([3, 8])
        .on_press(Message::NavigateTo(Screen::Login));

        let save_btn = {
            let label = if self.sheet_is_saving {
                "[ Salvando... ]"
            } else if self.sheet_saved {
                "[ Ficha Salva ✔ ]"
            } else {
                "[ Salvar Ficha ↵ ]"
            };
            let mut b = button(
                container(text(label).size(16).font(RETRO_UI_FONT)).align_x(Alignment::Center),
            )
            .style(default_button_style)
            .padding([3, 10]);
            if !self.sheet_is_saving {
                b = b.on_press(Message::SubmitCreateSheet);
            }
            b
        };

        let bio_editor = sunken_3d(
            text_editor(&self.sheet_bio)
                .font(RETRO_UI_FONT)
                .size(16)
                .height(68)
                .padding([4, 6])
                .style(next_text_editor_style)
                .on_action(Message::SheetBioAction),
            INPUT_BG,
            1.5,
        );

        let panel_biografia = sheet_section_panel(
            "3. Histórico do Personagem (Biografia)",
            RETRO_UI_FONT,
            column![
                bio_editor,
                Space::new().height(5),
                row![
                    help_btn,
                    Space::new().width(Length::Fill),
                    cancel_btn,
                    Space::new().width(6),
                    save_btn,
                ]
                .align_y(Alignment::Center)
                .width(Length::Fill),
            ],
            [6, 8],
        );

        let right_col = column![
            panel_identidade,
            Space::new().height(6),
            panel_equipamento,
            Space::new().height(6),
            panel_biografia,
        ]
        .width(Length::Fill);

        let main_columns = row![left_col, Space::new().width(8), right_col]
            .width(Length::Fill)
            .height(Length::Fill);

        let (status_msg, status_err) = &self.sheet_status;
        let bottom_footer = row![
            container(status_bar_box(
                text(format!("Status: {}", status_msg))
                    .size(16)
                    .font(RETRO_UI_FONT)
                    .color(status_color(*status_err)),
            ))
            .width(Length::Fill),
            Space::new().width(3),
            container(status_bar_box(
                text("RPGASYNCST Daemon v1.4")
                    .size(16)
                    .font(RETRO_UI_FONT)
                    .color(TEXT_DARK),
            ))
            .width(175),
        ]
        .align_y(Alignment::Center)
        .width(Length::Fill);

        container(column![
            container(main_columns)
                .padding([6, 8])
                .width(Length::Fill)
                .height(Length::Fill),
            bottom_footer,
        ])
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
