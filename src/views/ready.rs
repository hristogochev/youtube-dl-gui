use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, column, text};
use iced::{Alignment, Element, Length, Padding, Renderer};

use crate::gui::{Gui, Message};
use crate::resources::fonts::open_sans::{OPEN_SANS_BOLD, OPEN_SANS_SEMIBOLD};
use crate::resources::styles::button_styles::ButtonStyles;
use crate::resources::styles::text_styles::TextStyles;
use crate::resources::styles::theme::AppTheme;

impl Gui {
    /// Download ready view
    pub fn ready(&self) -> Element<'_, Message, Renderer<AppTheme>> {
        column(vec![])
            .push(
                text("Download complete")
                    .style(TextStyles::MainHeader)
                    .font(OPEN_SANS_BOLD)
                    .size(27),
            )
            .spacing(24)
            .push(
                button(
                    text("Convert more")
                        .size(20)
                        .style(TextStyles::ButtonText)
                        .font(OPEN_SANS_SEMIBOLD)
                        .horizontal_alignment(Horizontal::Center)
                        .vertical_alignment(Vertical::Center)
                        .width(Length::Fill),
                )
                .padding(Padding::from([10, 18]))
                .style(ButtonStyles::Continue)
                .on_press(Message::Reset),
            )
            .align_items(Alignment::Center)
            .into()
    }
}
