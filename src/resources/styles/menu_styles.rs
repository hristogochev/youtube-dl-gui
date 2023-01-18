use crate::resources::colors;
use crate::resources::styles::theme::AppTheme;
use iced::{overlay, Background};

impl overlay::menu::StyleSheet for AppTheme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> overlay::menu::Appearance {
        overlay::menu::Appearance {
            text_color: colors::white(),
            background: Background::Color(colors::background()),
            border_width: 1.0,
            border_radius: 4.0,
            border_color: colors::primary(),
            selected_text_color: colors::white(),
            selected_background: Background::Color(colors::background_highlight()),
        }
    }
}
