pub mod login;
pub mod player;
pub mod session;
pub mod sheet;
pub mod sprite_editor;

use std::borrow::Borrow;

use iced::widget::{container, pick_list, row, text, text_input};
use iced::{Alignment, Color, Element, Length};

use crate::ui::models::{Message, RETRO_UI_FONT};
use crate::ui::theme::{
    INPUT_BG, STATUS_ERROR, STATUS_SUCCESS, TEXT_DARK, login_input_style,
    next_pick_list_menu_style, sheet_pick_list_style, sunken_3d,
};

pub(crate) fn status_color(err: Option<bool>) -> Color {
    match err {
        Some(true) => STATUS_ERROR,
        Some(false) => STATUS_SUCCESS,
        None => TEXT_DARK,
    }
}

pub(crate) fn field_row<'a>(
    label: &'a str,
    label_width: f32,
    input: Element<'a, Message>,
) -> Element<'a, Message> {
    row![
        container(text(label).size(17).font(RETRO_UI_FONT).color(TEXT_DARK))
            .width(label_width)
            .align_y(Alignment::Center),
        input,
    ]
    .spacing(6)
    .align_y(Alignment::Center)
    .into()
}

pub(crate) fn attr_row<'a>(
    label: &'a str,
    value: &'a str,
    on_change: impl Fn(String) -> Message + 'a,
) -> Element<'a, Message> {
    row![
        container(text(label).size(15).font(RETRO_UI_FONT).color(TEXT_DARK))
            .width(Length::Fill)
            .align_y(Alignment::Center),
        container(sunken_3d(
            text_input("10", value)
                .font(RETRO_UI_FONT)
                .size(16)
                .style(login_input_style)
                .padding([1, 4])
                .on_input(on_change)
                .on_submit(Message::TabPressed(false)),
            INPUT_BG,
            1.5,
        ))
        .width(36),
    ]
    .spacing(4)
    .align_y(Alignment::Center)
    .into()
}

pub(crate) fn retro_pick_list<'a, T, L, V>(
    options: L,
    selected: Option<V>,
    on_selected: impl Fn(T) -> Message + 'a,
    text_size: u32,
) -> Element<'a, Message>
where
    T: ToString + PartialEq + Clone + 'a,
    L: Borrow<[T]> + 'a,
    V: Borrow<T> + 'a,
{
    pick_list(options, selected, on_selected)
        .font(RETRO_UI_FONT)
        .text_size(text_size)
        .width(Length::Fill)
        .padding([2, 6])
        .style(sheet_pick_list_style)
        .menu_style(next_pick_list_menu_style)
        .into()
}
