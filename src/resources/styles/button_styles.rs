use crate::resources::colors;
use crate::resources::styles::theme::AppTheme;
use iced::widget::button;
use iced::Background;

#[derive(Debug, Clone, Copy, Default)]
pub enum ButtonStyles {
    #[default]
    Default,
    Download,
    GoBack,
    Retry,
    Continue,
}

impl button::StyleSheet for AppTheme {
    type Style = ButtonStyles;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        match style {
            ButtonStyles::Default => button::Appearance::default(),
            ButtonStyles::Download => button::Appearance {
                shadow_offset: Default::default(),
                background: Some(Background::Color(colors::accent())),
                border_radius: 4.0,
                border_width: 0.0,
                border_color: Default::default(),
                text_color: colors::white(),
            },
            ButtonStyles::GoBack => button::Appearance {
                shadow_offset: Default::default(),
                background: Some(Background::Color(colors::accent())),
                border_radius: 4.0,
                border_width: 0.0,
                border_color: Default::default(),
                text_color: colors::white(),
            },
            ButtonStyles::Retry => button::Appearance {
                shadow_offset: Default::default(),
                background: Some(Background::Color(colors::secondary())),
                border_radius: 4.0,
                border_width: 0.0,
                border_color: Default::default(),
                text_color: colors::white(),
            },
            ButtonStyles::Continue => button::Appearance {
                shadow_offset: Default::default(),
                background: Some(Background::Color(colors::accent())),
                border_radius: 4.0,
                border_width: 0.0,
                border_color: Default::default(),
                text_color: colors::white(),
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        match style {
            ButtonStyles::Default => button::Appearance::default(),
            ButtonStyles::Download => button::Appearance {
                shadow_offset: Default::default(),
                background: Some(Background::Color(colors::accent_highlight())),
                border_radius: 4.0,
                border_width: 0.0,
                border_color: Default::default(),
                text_color: colors::white(),
            },
            ButtonStyles::GoBack => button::Appearance {
                shadow_offset: Default::default(),
                background: Some(Background::Color(colors::accent_highlight())),
                border_radius: 4.0,
                border_width: 0.0,
                border_color: Default::default(),
                text_color: colors::white(),
            },
            ButtonStyles::Retry => button::Appearance {
                shadow_offset: Default::default(),
                background: Some(Background::Color(colors::secondary_highlight())),
                border_radius: 4.0,
                border_width: 0.0,
                border_color: Default::default(),
                text_color: colors::white(),
            },
            ButtonStyles::Continue => button::Appearance {
                shadow_offset: Default::default(),
                background: Some(Background::Color(colors::accent_highlight())),
                border_radius: 4.0,
                border_width: 0.0,
                border_color: Default::default(),
                text_color: colors::white(),
            },
        }
    }
}
