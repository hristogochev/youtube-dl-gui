use crate::resources::styles::theme::AppTheme;
use iced::widget::container;

impl container::StyleSheet for AppTheme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance::default()
    }
}
