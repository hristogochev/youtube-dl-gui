use crate::resources::colors;
use crate::resources::styles::theme::AppTheme;
use iced::widget::progress_bar;
use iced::Background;

impl progress_bar::StyleSheet for AppTheme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> progress_bar::Appearance {
        progress_bar::Appearance {
            background: Background::Color(colors::background_highlight()),
            bar: Background::Color(colors::accent()),
            border_radius: 5.0,
        }
    }
}
