use crate::resources::colors;
use crate::resources::styles::theme::AppTheme;
use iced::widget::pick_list;
use iced::Background;

impl pick_list::StyleSheet for AppTheme {
    type Style = ();

    fn active(&self, _style: &<Self as pick_list::StyleSheet>::Style) -> pick_list::Appearance {
        pick_list::Appearance {
            text_color: colors::white(),
            placeholder_color: colors::primary(),
            handle_color: colors::white(),
            background: Background::Color(colors::background()),
            border_radius: 4.0,
            border_width: 1.0,
            border_color: colors::primary(),
        }
    }

    fn hovered(&self, _style: &<Self as pick_list::StyleSheet>::Style) -> pick_list::Appearance {
        pick_list::Appearance {
            text_color: colors::white(),
            placeholder_color: colors::primary(),
            handle_color: colors::white(),
            background: Background::Color(colors::background_highlight()),
            border_radius: 4.0,
            border_width: 1.0,
            border_color: colors::primary(),
        }
    }
}
