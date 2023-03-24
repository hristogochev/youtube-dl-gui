use iced::alignment::{Horizontal, Vertical};
use iced::widget::pick_list;
use iced::widget::text_input::Id;
use iced::widget::{button, column, text, text_input};
use iced::{Alignment, Element, Length, Padding, Renderer};

use crate::gui::{Gui, Message};
use crate::resources::fonts::open_sans::{OPEN_SANS_BOLD, OPEN_SANS_SEMIBOLD};
use crate::resources::styles::button_styles::ButtonStyles;
use crate::resources::styles::text_styles::TextStyles;
use crate::resources::styles::theme::AppTheme;
use crate::utils::format::Format;

impl Gui {
    /// User input view
    pub fn choosing(&self) -> Element<'_, Message, Renderer<AppTheme>> {
        column(vec![])
            .push(
                text("YouTube Converter")
                    .style(TextStyles::MainHeader)
                    .font(OPEN_SANS_BOLD)
                    .size(27),
            )
            .spacing(26)
            .push(
                column(vec![])
                    .push(
                        column(vec![])
                            .push(
                                column(vec![])
                                    .push(
                                        text("Video url:")
                                            .size(22)
                                            .style(TextStyles::Label)
                                            .font(OPEN_SANS_SEMIBOLD),
                                    )
                                    .spacing(7)
                                    .push(
                                        text_input(
                                            "Paste video url...",
                                            &self.url,
                                            Message::UrlChanged,
                                        )
                                        .padding(Padding::from([8, 10]))
                                        .size(21)
                                        .id(Id::new("video_url_input")),
                                    ),
                            )
                            .spacing(13)
                            .push(
                                column(vec![])
                                    .push(
                                        text("Format:")
                                            .size(22)
                                            .style(TextStyles::Label)
                                            .font(OPEN_SANS_SEMIBOLD),
                                    )
                                    .spacing(7)
                                    .push(
                                        pick_list(
                                            &Format::ALL[..],
                                            self.format,
                                            Message::FormatChanged,
                                        )
                                        .width(Length::Fill)
                                        .placeholder("Choose a format...")
                                        .padding(Padding::from([8, 10]))
                                        .text_size(21)
                                        .handle(pick_list::Handle::Arrow { size: Some(28.0) }),
                                    ),
                            ),
                    )
                    .spacing(24)
                    .push(
                        button(
                            text("Download")
                                .size(22)
                                .style(TextStyles::ButtonText)
                                .font(OPEN_SANS_SEMIBOLD)
                                .horizontal_alignment(Horizontal::Center)
                                .vertical_alignment(Vertical::Center)
                                .width(Length::Fill),
                        )
                        .width(Length::Fill)
                        .padding(Padding::from([10, 0]))
                        .style(ButtonStyles::Download)
                        .on_press(Message::SelectingDir),
                    ),
            )
            .align_items(Alignment::Center)
            .into()
    }
}
