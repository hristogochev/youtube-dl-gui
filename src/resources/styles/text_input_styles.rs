use crate::resources::colors;
use crate::resources::styles::theme::AppTheme;
use iced::widget::text_input;
use iced::{Background, Color};

impl text_input::StyleSheet for AppTheme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(colors::background()),
            border_radius: 4.0,
            border_width: 1.0,
            border_color: colors::primary(),
        }
    }

    fn focused(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(colors::background()),
            border_radius: 4.0,
            border_width: 1.0,
            border_color: colors::primary(),
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        colors::primary()
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        colors::white()
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        colors::selection()
    }
}
