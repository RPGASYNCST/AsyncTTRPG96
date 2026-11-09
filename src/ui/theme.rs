use iced::border::Border;
use iced::widget::{button, container, text_input};
use iced::{Color, Element, Length, Padding, Shadow, Vector};

pub const CDE_TEAL_MAIN: Color = Color::from_rgb(0.08, 0.40, 0.45);
pub const CDE_TEAL_LIGHT: Color = Color::from_rgb(0.35, 0.62, 0.66);
pub const CDE_TEAL_DARK: Color = Color::from_rgb(0.03, 0.22, 0.25);

pub const CDE_GRAY: Color = Color::from_rgb(0.75, 0.75, 0.77);
pub const CDE_DARK_GRAY: Color = Color::from_rgb(0.45, 0.46, 0.48);
pub const CDE_BEVEL_LIGHT: Color = Color::from_rgb(0.95, 0.95, 0.97);
pub const CDE_BEVEL_DARK: Color = Color::from_rgb(0.22, 0.23, 0.25);

pub const INPUT_SUNKEN_BG: Color = Color::from_rgb(0.58, 0.59, 0.61)
pub const TEXT_DARK: Color = Color::from_rgb(0.04, 0.04, 0.04);
pub const TEXT_LIGHT: Color = Color::from_rgb(0.96, 0.97, 0.98);

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
            background: Some(CDE_BEVEL_DARK.into()),
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
        background: Some(CDE_BEVEL_LIGHT.into()),
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
                background: Some(CDE_GRAY.into()),
                border: Border {
                    color: CDE_BEVEL_LIGHT,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            }),
    )
    .width(Length::Fill)
    .padding(1.0)
    .style(|_t| container::Style {
        background: Some(CDE_DARK_GRAY.into()),
        border: Border {
            color: CDE_DARK_GRAY,
            width: 1.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    })
    .into()
}

pub fn outer_window_frame(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(CDE_GRAY.into()),
        text_color: Some(TEXT_DARK),
        border: Border {
            color: CDE_TEAL_MAIN,
            width: 3.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

pub fn title_bar_segment(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(CDE_TEAL_MAIN.into()),
        text_color: Some(TEXT_LIGHT),
        border: Border {
            color: CDE_TEAL_LIGHT,
            width: 1.5,
            radius: 0.0.into(),
        },
        shadow: Shadow {
            color: CDE_TEAL_DARK,
            offset: Vector::new(1.0, 1.0),
            blur_radius: 0.0,
        },
        ..Default::default()
    }
}

pub fn title_button_style(_theme: &iced::Theme, status: button::Status) -> button::Style {
    let bg = match status {
        button::Status::Pressed => CDE_TEAL_DARK,
        _ => CDE_TEAL_MAIN,
    };
    button::Style {
        background: Some(bg.into()),
        text_color: TEXT_LIGHT,
        border: Border {
            color: CDE_TEAL_LIGHT,
            width: 1.5,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

pub fn login_input_style(_theme: &iced::Theme, _status: text_input::Status) -> text_input::Style {
    text_input::Style {
        background: INPUT_SUNKEN_BG.into(),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 0.0.into(),
        },
        icon: TEXT_DARK,
        placeholder: CDE_DARK_GRAY,
        value: TEXT_DARK,
        selection: Color::from_rgb(0.10, 0.30, 0.45),
    }
}

pub fn login_button_style(_theme: &iced::Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Pressed => button::Style {
            background: Some(CDE_DARK_GRAY.into()),
            text_color: TEXT_DARK,
            border: Border {
                color: CDE_BEVEL_DARK,
                width: 1.5,
                radius: 0.0.into(),
            },
            ..Default::default()
        },
        button::Status::Hovered => button::Style {
            background: Some(Color::from_rgb(0.80, 0.80, 0.82).into()),
            text_color: TEXT_DARK,
            border: Border {
                color: CDE_BEVEL_LIGHT,
                width: 1.5,
                radius: 0.0.into(),
            },
            shadow: Shadow {
                color: CDE_BEVEL_DARK,
                offset: Vector::new(1.5, 1.5),
                blur_radius: 0.0,
            },
            ..Default::default()
        },
        _ => button::Style {
            background: Some(CDE_GRAY.into()),
            text_color: TEXT_DARK,
            border: Border {
                color: CDE_BEVEL_LIGHT,
                width: 1.5,
                radius: 0.0.into(),
            },
            shadow: Shadow {
                color: CDE_BEVEL_DARK,
                offset: Vector::new(1.5, 1.5),
                blur_radius: 0.0,
            },
            ..Default::default()
        },
    }
}
