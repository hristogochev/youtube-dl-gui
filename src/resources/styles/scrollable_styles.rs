use crate::resources::colors;
use crate::resources::styles::theme::AppTheme;
use iced::widget::scrollable;

impl scrollable::StyleSheet for AppTheme {
    type Style = ();

    fn active(&self, _style: &Self::Style) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            background: None,
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Default::default(),
            scroller: scrollable::Scroller {
                color: colors::primary(),
                border_radius: 4.0,
                border_width: 0.0,
                border_color: Default::default(),
            },
        }
    }

    fn hovered(&self, _style: &Self::Style) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            background: None,
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Default::default(),
            scroller: scrollable::Scroller {
                color: colors::primary(),
                border_radius: 4.0,
                border_width: 0.0,
                border_color: Default::default(),
            },
        }
    }
}
