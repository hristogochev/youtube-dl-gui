use crate::resources::styles::theme::AppTheme;
use iced::widget::rule;
use iced::widget::rule::FillMode;

impl rule::StyleSheet for AppTheme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> rule::Appearance {
        rule::Appearance {
            color: Default::default(),
            width: 0,
            radius: 0.0,
            fill_mode: FillMode::Full,
        }
    }
}
