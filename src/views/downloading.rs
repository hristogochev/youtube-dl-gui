use iced::widget::{column, progress_bar, text};
use iced::{Alignment, Element, Length, Renderer};

use crate::gui::{Gui, Message};
use crate::resources::fonts::open_sans::{OPEN_SANS_BOLD, OPEN_SANS_SEMIBOLD};
use crate::resources::styles::text_styles::TextStyles;
use crate::resources::styles::theme::AppTheme;

impl Gui {
    /// Download currently in progress view
    pub fn downloading(
        &self,
        state: &str,
        progress: f32,
        _total_size: u32,
        _download_speed: u32,
        _eta: u32,
    ) -> Element<'_, Message, Renderer<AppTheme>> {
        column(vec![])
            .push(
                text(state)
                    .style(TextStyles::MainHeader)
                    .font(OPEN_SANS_BOLD)
                    .size(27),
            )
            .spacing(20)
            .push(
                column(vec![])
                    .push(
                        progress_bar(0.0..=100.0, progress)
                            .width(Length::Fixed(200.0))
                            .height(Length::Fixed(12.0)),
                    )
                    .spacing(10)
                    .push(
                        text(format!("{}%", progress.floor()))
                            .style(TextStyles::Progress)
                            .font(OPEN_SANS_SEMIBOLD)
                            .size(21),
                    )
                    .align_items(Alignment::Center),
            )
            .align_items(Alignment::Center)
            .into()
    }
}
