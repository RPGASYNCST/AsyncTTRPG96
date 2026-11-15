use iced::border::Border;
use iced::overlay::menu;
use iced::widget::{
    Space, button, column, container, pick_list, radio, row, stack, text, text_editor, text_input,
};
use iced::{Color, Element, Length, Padding, Shadow, Vector};

pub const NEXT_BLACK: Color = Color::from_rgb(0.0, 0.0, 0.0);

pub const NEXT_DARK_GRAPHITE: Color = Color::from_rgb(0.18, 0.18, 0.18);

pub const NEXT_MID_GRAY: Color = Color::from_rgb(0.48, 0.48, 0.48);

pub const NEXT_WINDOW_GRAY: Color = Color::from_rgb(0.70, 0.70, 0.70);

pub const NEXT_BUTTON_GRAY: Color = Color::from_rgb(0.82, 0.82, 0.82);

pub const NEXT_WHITE: Color = Color::from_rgb(1.0, 1.0, 1.0);

pub const NEXT_BEVEL_LIGHT: Color = Color::from_rgb(1.0, 1.0, 1.0);

pub const NEXT_BEVEL_DARK: Color = Color::from_rgb(0.30, 0.30, 0.30);

pub const TEXT_DARK: Color = Color::from_rgb(0.0, 0.0, 0.0);
pub const TEXT_LIGHT: Color = Color::from_rgb(1.0, 1.0, 1.0);
pub const TEXT_MUTED: Color = Color::from_rgb(0.35, 0.35, 0.35);

pub const INPUT_BG: Color = Color::from_rgb(1.0, 1.0, 1.0);

pub const STATUS_ERROR: Color = Color::from_rgb(0.65, 0.08, 0.08);
pub const STATUS_SUCCESS: Color = Color::from_rgb(0.08, 0.45, 0.12);

pub fn sunken_3d<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    bg: Color,
    bevel_px: f32,
) -> Element<'a, Message> {
    container(
        container(
            container(content)
                .width(Length::Fill)
                .style(move |_t| container::Style {
                    background: Some(bg.into()),
                    ..Default::default()
                }),
        )
        .width(Length::Fill)
        .padding(Padding {
            top: bevel_px,
            left: bevel_px,
            right: 0.0,
            bottom: 0.0,
        })
        .style(|_t| container::Style {
            background: Some(NEXT_DARK_GRAPHITE.into()),
            ..Default::default()
        }),
    )
    .width(Length::Fill)
    .padding(Padding {
        top: 0.0,
        left: 0.0,
        right: bevel_px,
        bottom: bevel_px,
    })
    .style(|_t| container::Style {
        background: Some(NEXT_BEVEL_LIGHT.into()),
        border: Border {
            color: NEXT_BLACK,
            width: 1.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    })
    .into()
}

pub fn etched_panel<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    padding: impl Into<Padding>,
) -> Element<'a, Message> {
    container(
        container(content)
            .width(Length::Fill)
            .padding(padding)
            .style(|_t| container::Style {
                background: Some(NEXT_WINDOW_GRAY.into()),
                border: Border {
                    color: NEXT_BEVEL_LIGHT,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            }),
    )
    .width(Length::Fill)
    .padding(1.0)
    .style(|_t| container::Style {
        background: Some(NEXT_MID_GRAY.into()),
        border: Border {
            color: NEXT_MID_GRAY,
            width: 1.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    })
    .into()
}

pub fn outer_window_frame(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(NEXT_WINDOW_GRAY.into()),
        text_color: Some(TEXT_DARK),
        border: Border {
            color: NEXT_BLACK,
            width: 2.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

pub fn title_bar_segment(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(NEXT_BLACK.into()),
        text_color: Some(TEXT_LIGHT),
        border: Border {
            color: NEXT_BLACK,
            width: 1.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

pub fn title_button_style(_theme: &iced::Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Pressed => button::Style {
            background: Some(NEXT_DARK_GRAPHITE.into()),
            text_color: TEXT_LIGHT,
            border: Border {
                color: NEXT_BLACK,
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        },
        button::Status::Hovered => button::Style {
            background: Some(NEXT_BUTTON_GRAY.into()),
            text_color: TEXT_DARK,
            border: Border {
                color: NEXT_BLACK,
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        },
        _ => button::Style {
            background: Some(NEXT_WINDOW_GRAY.into()),
            text_color: TEXT_DARK,
            border: Border {
                color: NEXT_BLACK,
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        },
    }
}

pub fn login_input_style(_theme: &iced::Theme, _status: text_input::Status) -> text_input::Style {
    text_input::Style {
        background: INPUT_BG.into(),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 0.0.into(),
        },
        icon: TEXT_DARK,
        placeholder: NEXT_MID_GRAY,
        value: TEXT_DARK,
        selection: Color::from_rgb(0.18, 0.18, 0.18),
    }
}

pub fn login_button_style(_theme: &iced::Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Pressed => button::Style {
            background: Some(Color::from_rgb(0.60, 0.60, 0.60).into()),
            text_color: TEXT_DARK,
            border: Border {
                color: NEXT_BLACK,
                width: 1.0,
                radius: 0.0.into(),
            },
            shadow: Shadow {
                color: NEXT_DARK_GRAPHITE,
                offset: Vector::new(1.0, 1.0),
                blur_radius: 0.0,
            },
            ..Default::default()
        },
        button::Status::Hovered => button::Style {
            background: Some(Color::from_rgb(0.88, 0.88, 0.88).into()),
            text_color: TEXT_DARK,
            border: Border {
                color: NEXT_BLACK,
                width: 1.0,
                radius: 0.0.into(),
            },
            shadow: Shadow {
                color: NEXT_DARK_GRAPHITE,
                offset: Vector::new(1.5, 1.5),
                blur_radius: 0.0,
            },
            ..Default::default()
        },
        _ => button::Style {
            background: Some(NEXT_BUTTON_GRAY.into()),
            text_color: TEXT_DARK,
            border: Border {
                color: NEXT_BLACK,
                width: 1.0,
                radius: 0.0.into(),
            },
            shadow: Shadow {
                color: NEXT_DARK_GRAPHITE,
                offset: Vector::new(1.5, 1.5),
                blur_radius: 0.0,
            },
            ..Default::default()
        },
    }
}

pub fn default_button_style(_theme: &iced::Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Pressed => button::Style {
            background: Some(Color::from_rgb(0.55, 0.55, 0.55).into()),
            text_color: TEXT_DARK,
            border: Border {
                color: NEXT_BLACK,
                width: 2.0,
                radius: 0.0.into(),
            },
            shadow: Shadow {
                color: NEXT_DARK_GRAPHITE,
                offset: Vector::new(1.0, 1.0),
                blur_radius: 0.0,
            },
            ..Default::default()
        },
        button::Status::Hovered => button::Style {
            background: Some(Color::from_rgb(0.90, 0.90, 0.90).into()),
            text_color: TEXT_DARK,
            border: Border {
                color: NEXT_BLACK,
                width: 2.0,
                radius: 0.0.into(),
            },
            shadow: Shadow {
                color: NEXT_DARK_GRAPHITE,
                offset: Vector::new(1.5, 1.5),
                blur_radius: 0.0,
            },
            ..Default::default()
        },
        _ => button::Style {
            background: Some(NEXT_BUTTON_GRAY.into()),
            text_color: TEXT_DARK,
            border: Border {
                color: NEXT_BLACK,
                width: 2.0,
                radius: 0.0.into(),
            },
            shadow: Shadow {
                color: NEXT_DARK_GRAPHITE,
                offset: Vector::new(1.5, 1.5),
                blur_radius: 0.0,
            },
            ..Default::default()
        },
    }
}

pub fn group_box<'a, Message: 'a>(
    title: &'a str,
    font: iced::Font,
    content: impl Into<Element<'a, Message>>,
    inner_padding: impl Into<Padding>,
) -> Element<'a, Message> {
    let box_body = container(etched_panel(content, inner_padding))
        .padding(Padding {
            top: 9.0,
            right: 0.0,
            bottom: 0.0,
            left: 0.0,
        })
        .width(Length::Fill);

    let title_tag = row![
        Space::new().width(10),
        container(text(title).size(17).font(font).color(TEXT_DARK))
            .padding([0, 5])
            .style(|_t| container::Style {
                background: Some(NEXT_WINDOW_GRAY.into()),
                ..Default::default()
            }),
    ];

    stack![box_body, title_tag].width(Length::Fill).into()
}

pub fn status_bar_box<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    container(
        container(content)
            .width(Length::Fill)
            .padding([3, 8])
            .style(|_t| container::Style {
                background: Some(NEXT_WINDOW_GRAY.into()),
                border: Border {
                    color: NEXT_BEVEL_LIGHT,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            }),
    )
    .width(Length::Fill)
    .padding(Padding {
        top: 1.0,
        left: 1.0,
        right: 0.0,
        bottom: 0.0,
    })
    .style(|_t| container::Style {
        background: Some(NEXT_MID_GRAY.into()),
        border: Border {
            color: NEXT_MID_GRAY,
            width: 1.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    })
    .into()
}

pub fn next_radio_style(_theme: &iced::Theme, status: radio::Status) -> radio::Style {
    let hovered = matches!(status, radio::Status::Hovered { .. });
    radio::Style {
        background: if hovered {
            NEXT_BUTTON_GRAY.into()
        } else {
            NEXT_WHITE.into()
        },
        dot_color: NEXT_BLACK,
        border_width: 1.5,
        border_color: NEXT_BLACK,
        text_color: Some(TEXT_DARK),
    }
}

pub fn next_pick_list_style(_theme: &iced::Theme, status: pick_list::Status) -> pick_list::Style {
    let bg = match status {
        pick_list::Status::Hovered | pick_list::Status::Opened { .. } => NEXT_BUTTON_GRAY,
        _ => INPUT_BG,
    };
    pick_list::Style {
        text_color: TEXT_DARK,
        placeholder_color: NEXT_MID_GRAY,
        handle_color: NEXT_BLACK,
        background: bg.into(),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 0.0.into(),
        },
    }
}

pub fn horizontal_groove<'a, Message: 'a>() -> Element<'a, Message> {
    column![
        container(Space::new().height(1).width(Length::Fill)).style(|_t| container::Style {
            background: Some(NEXT_MID_GRAY.into()),
            ..Default::default()
        }),
        container(Space::new().height(1).width(Length::Fill)).style(|_t| container::Style {
            background: Some(NEXT_BEVEL_LIGHT.into()),
            ..Default::default()
        }),
    ]
    .width(Length::Fill)
    .into()
}

pub fn sheet_section_panel<'a, Message: 'a>(
    header_title: &'a str,
    font: iced::Font,
    body: impl Into<Element<'a, Message>>,
    body_padding: impl Into<Padding>,
) -> Element<'a, Message> {
    etched_panel(
        column![
            container(text(header_title).size(19).font(font).color(TEXT_DARK)).padding([4, 8]),
            horizontal_groove(),
            container(body).padding(body_padding).width(Length::Fill),
        ]
        .width(Length::Fill),
        0,
    )
}

pub fn next_text_editor_style(
    _theme: &iced::Theme,
    _status: text_editor::Status,
) -> text_editor::Style {
    text_editor::Style {
        background: INPUT_BG.into(),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 0.0.into(),
        },
        placeholder: NEXT_MID_GRAY,
        value: TEXT_DARK,
        selection: Color::from_rgb(0.75, 0.75, 0.75),
    }
}

pub fn sheet_pick_list_style(_theme: &iced::Theme, status: pick_list::Status) -> pick_list::Style {
    let bg = match status {
        pick_list::Status::Hovered | pick_list::Status::Opened { .. } => NEXT_BUTTON_GRAY,
        _ => Color::from_rgb(0.76, 0.76, 0.76),
    };
    pick_list::Style {
        text_color: TEXT_DARK,
        placeholder_color: NEXT_MID_GRAY,
        handle_color: NEXT_BLACK,
        background: bg.into(),
        border: Border {
            color: NEXT_BLACK,
            width: 1.0,
            radius: 0.0.into(),
        },
    }
}
pub fn next_pick_list_menu_style(_theme: &iced::Theme) -> menu::Style {
    menu::Style {
        background: INPUT_BG.into(),
        border: Border {
            color: NEXT_BLACK,
            width: 1.5,
            radius: 0.0.into(),
        },
        text_color: TEXT_DARK,
        selected_text_color: TEXT_LIGHT,
        selected_background: NEXT_DARK_GRAPHITE.into(),
        shadow: Shadow::default(),
    }
}
