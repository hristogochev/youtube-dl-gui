use crate::resources::colors;
use crate::resources::styles::theme::AppTheme;
use iced::widget::text;

#[derive(Debug, Clone, Copy, Default)]
pub enum TextStyles {
    #[default]
    Default,
    MainHeader,
    Label,
    Progress,
    ButtonText,
}

impl text::StyleSheet for AppTheme {
    type Style = TextStyles;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        match style {
            TextStyles::Default => text::Appearance::default(),
            TextStyles::MainHeader => text::Appearance {
                color: Some(colors::white()),
            },
            TextStyles::Label => text::Appearance {
                color: Some(colors::white()),
            },
            TextStyles::Progress => text::Appearance {
                color: Some(colors::white()),
            },
            TextStyles::ButtonText => text::Appearance {
                color: Some(colors::white()),
            },
        }
    }
}
