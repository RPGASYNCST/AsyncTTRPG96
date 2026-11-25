use iced::widget::{Space, button, column, container, image, row, text};
use iced::{Alignment, Element, Length};

use crate::ui::app::RetroApp;
use crate::ui::models::{Message, RETRO_UI_FONT, Screen};
use crate::ui::sprite::{
    CloakColor, FacialFeature, HairColor, HairStyle, HeadFeature, OutfitColor, OutfitStyle,
    SkinTone, WeaponType,
};
use crate::ui::theme::{
    NEXT_WINDOW_GRAY, TEXT_DARK, TEXT_MUTED, default_button_style, login_button_style,
    sheet_section_panel, status_bar_box, sunken_3d,
};
use crate::ui::views::{field_row, retro_pick_list};

impl RetroApp {
    pub(crate) fn view_sprite_editor(&self) -> Element<'_, Message> {
        let preview_img = image(self.sprite_preview_handle.clone())
            .width(136)
            .height(152)
            .filter_method(image::FilterMethod::Nearest);

        let display_char_name = if self.sheet_name.trim().is_empty() {
            "Aventureiro"
        } else {
            self.sheet_name.trim()
        };

        let random_btn = button(
            container(text("[ Aleatório 🎲 ]").size(16).font(RETRO_UI_FONT))
                .width(Length::Fill)
                .align_x(Alignment::Center),
        )
        .style(login_button_style)
        .width(Length::Fill)
        .padding([4, 6])
        .on_press(Message::SpriteRandomize);

        let reset_btn = button(
            container(text("[ Restaurar Padrão ]").size(16).font(RETRO_UI_FONT))
                .width(Length::Fill)
                .align_x(Alignment::Center),
        )
        .style(login_button_style)
        .width(Length::Fill)
        .padding([4, 6])
        .on_press(Message::SpriteResetDefault);

        let preview_panel = sheet_section_panel(
            "Pré-Visualização 16-Bit",
            RETRO_UI_FONT,
            column![
                sunken_3d(
                    container(preview_img)
                        .width(Length::Fill)
                        .padding([8, 6])
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center),
                    NEXT_WINDOW_GRAY,
                    1.5,
                ),
                Space::new().height(6),
                container(
                    text(display_char_name)
                        .size(20)
                        .font(RETRO_UI_FONT)
                        .color(TEXT_DARK)
                )
                .width(Length::Fill)
                .align_x(Alignment::Center),
                container(
                    text(format!(
                        "{} • {}",
                        self.sheet_race.as_str(),
                        self.sheet_class.as_str()
                    ))
                    .size(15)
                    .font(RETRO_UI_FONT)
                    .color(TEXT_MUTED)
                )
                .width(Length::Fill)
                .align_x(Alignment::Center),
                Space::new().height(10),
                random_btn,
                Space::new().height(5),
                reset_btn,
            ]
            .align_x(Alignment::Center),
            [8, 10],
        );

        let left_col = column![preview_panel].width(185);

        let group_cabeca = sheet_section_panel(
            "1. Anatomia, Rosto & Cabelo",
            RETRO_UI_FONT,
            column![
                field_row(
                    "Tom de Pele:",
                    138.0,
                    retro_pick_list(
                        SkinTone::ALL,
                        Some(self.sprite_config.skin),
                        Message::SpriteSkinChanged,
                        16,
                    )
                ),
                field_row(
                    "Orelhas / Traços:",
                    138.0,
                    retro_pick_list(
                        HeadFeature::ALL,
                        Some(self.sprite_config.head_feature),
                        Message::SpriteHeadChanged,
                        16,
                    )
                ),
                field_row(
                    "Cabelo / Elmo:",
                    138.0,
                    retro_pick_list(
                        HairStyle::ALL,
                        Some(self.sprite_config.hair_style),
                        Message::SpriteHairStyleChanged,
                        16,
                    )
                ),
                field_row(
                    "Cor do Cabelo:",
                    138.0,
                    retro_pick_list(
                        HairColor::ALL,
                        Some(self.sprite_config.hair_color),
                        Message::SpriteHairColorChanged,
                        16,
                    )
                ),
                field_row(
                    "Barba / Rosto:",
                    138.0,
                    retro_pick_list(
                        FacialFeature::ALL,
                        Some(self.sprite_config.facial),
                        Message::SpriteFacialChanged,
                        16,
                    )
                ),
            ]
            .spacing(4),
            [6, 8],
        );

        let cancel_btn = button(
            container(text("[ Voltar p/ Ficha ]").size(16).font(RETRO_UI_FONT))
                .align_x(Alignment::Center),
        )
        .style(login_button_style)
        .padding([4, 10])
        .on_press(Message::NavigateTo(Screen::CreateSheet));

        let apply_btn = button(
            container(text("[ Aplicar Sprite ↵ ]").size(16).font(RETRO_UI_FONT))
                .align_x(Alignment::Center),
        )
        .style(default_button_style)
        .padding([4, 12])
        .on_press(Message::SpriteApplyToSheet);

        let group_equipamento = sheet_section_panel(
            "2. Traje, Armadura, Capa & Armamento",
            RETRO_UI_FONT,
            column![
                field_row(
                    "Traje / Armadura:",
                    138.0,
                    retro_pick_list(
                        OutfitStyle::ALL,
                        Some(self.sprite_config.outfit_style),
                        Message::SpriteOutfitStyleChanged,
                        16,
                    )
                ),
                field_row(
                    "Cor Principal:",
                    138.0,
                    retro_pick_list(
                        OutfitColor::ALL,
                        Some(self.sprite_config.outfit_color),
                        Message::SpriteOutfitColorChanged,
                        16,
                    )
                ),
                field_row(
                    "Capa / Costas:",
                    138.0,
                    retro_pick_list(
                        CloakColor::ALL,
                        Some(self.sprite_config.cloak),
                        Message::SpriteCloakChanged,
                        16,
                    )
                ),
                field_row(
                    "Arma Equipada:",
                    138.0,
                    retro_pick_list(
                        WeaponType::ALL,
                        Some(self.sprite_config.weapon),
                        Message::SpriteWeaponChanged,
                        16,
                    )
                ),
                Space::new().height(8),
                row![
                    Space::new().width(Length::Fill),
                    cancel_btn,
                    Space::new().width(8),
                    apply_btn,
                ]
                .align_y(Alignment::Center)
                .width(Length::Fill),
            ]
            .spacing(4),
            [6, 8],
        );

        let right_col =
            column![group_cabeca, Space::new().height(6), group_equipamento].width(Length::Fill);

        let main_columns = row![left_col, Space::new().width(8), right_col]
            .width(Length::Fill)
            .height(Length::Fill);

        let bottom_footer = row![
            container(status_bar_box(
                text(format!("Status: {}", self.sprite_status))
                    .size(16)
                    .font(RETRO_UI_FONT)
                    .color(TEXT_DARK),
            ))
            .width(Length::Fill),
            Space::new().width(3),
            container(status_bar_box(
                text("Sprite Engine 16-Bit v1.4")
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
