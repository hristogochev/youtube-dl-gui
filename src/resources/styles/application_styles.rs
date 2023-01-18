use crate::resources::colors;
use crate::resources::styles::theme::AppTheme;
use iced::application;

impl application::StyleSheet for AppTheme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: colors::background(),
            text_color: Default::default(),
        }
    }
}
