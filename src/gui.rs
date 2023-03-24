use iced::widget::text_input::Id;
use iced::widget::{container, text_input};
use iced::window::Position;
use iced::{
    executor, subscription, window, Application, Command, Element, Length, Padding, Renderer,
    Settings, Subscription,
};
use log::{error, warn};

use crate::resources::fonts::open_sans::RAW_OPEN_SANS_REGULAR;
use crate::resources::images::get_icon;
use crate::resources::styles::theme::AppTheme;
use crate::utils::format::Format;
use crate::utils::parsed_output_stream::parsed_output_stream;
use crate::utils::{download_path_picker, query_cache};

pub struct Gui {
    pub dir: Option<String>,
    pub url: String,
    pub format: Option<Format>,
    pub state: State,
}

pub enum State {
    Choosing,
    Preparing,
    Downloading(f32, u32, u32, u32),
    Converting,
    Ready,
    Failed,
}

#[derive(Debug, Clone)]
pub enum Message {
    UrlChanged(String),
    FormatChanged(Format),
    SelectingDir,
    SelectedDir(Option<String>),
    Empty,
    Downloading(f32, u64, u64, u64),
    Downloaded(f32, u64, u64),
    Converting(String),
    Saved,
    Failed(String),
    DownloadRetry,
    Reset,
}

impl Gui {
    pub fn start() -> iced::Result {
        let icon = match get_icon() {
            Ok(icon) => icon,
            Err(err) => {
                error!("{err}");
                return Ok(());
            }
        };

        let window_settings = window::Settings {
            size: (360, 320),
            position: Position::Centered,
            min_size: None,
            max_size: None,
            visible: true,
            resizable: false,
            decorations: true,
            transparent: false,
            always_on_top: false,
            icon: Some(icon),
        };

        let settings = Settings {
            id: None,
            window: window_settings,
            flags: Default::default(),
            default_font: Some(RAW_OPEN_SANS_REGULAR),
            default_text_size: 20.0,
            text_multithreading: false,
            antialiasing: false,
            exit_on_close_request: true,
            try_opengles_first: false,
        };

        Gui::run(settings)
    }

    pub fn start_download(&mut self) {
        // If we start a download we clone the url, destination dir and format
        let url = if !self.url.is_empty() {
            self.url.clone()
        } else {
            warn!("Url is empty");
            return;
        };
        let Some(dir) = self.dir.as_ref().cloned() else {
            warn!("Dir is empty");
            return;
        };
        let Some(format)= self.format.as_ref().cloned() else {
            warn!("Format is empty");
            return;
        };

        // Then we put them in the query cache
        if let Err(err) = query_cache::replace_global(Some((url, dir, format))) {
            error!("Unable to replace cache: {err}");
            return;
        }

        // We set the state of the GUI to start the download subscription
        self.state = State::Preparing;
    }
}

impl Application for Gui {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = AppTheme;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        let gui = Gui {
            dir: None,
            url: "".to_owned(),
            format: Some(Format::Mp3),
            state: State::Choosing,
        };
        let first_command = text_input::focus(Id::new("video_url_input"));

        (gui, first_command)
    }

    fn title(&self) -> String {
        String::from("YouTube Converter")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::UrlChanged(url) => {
                self.url = url;
                Command::none()
            }
            Message::FormatChanged(format) => {
                self.format = Some(format);
                Command::none()
            }
            Message::SelectingDir => Command::perform(
                download_path_picker::select_download_path(),
                Message::SelectedDir,
            ),
            Message::SelectedDir(dir) => {
                self.dir = dir;
                self.start_download();
                Command::none()
            }
            Message::DownloadRetry => {
                self.start_download();
                Command::none()
            }
            Message::Downloading(progress, _, _, _) => {
                self.state = State::Downloading(progress, 0, 0, 0);
                Command::none()
            }
            Message::Downloaded(progress, _, _) => {
                self.state = State::Downloading(progress, 0, 0, 0);
                Command::none()
            }
            Message::Converting(_) => {
                self.state = State::Converting;
                Command::none()
            }
            Message::Saved => {
                self.state = State::Ready;
                Command::none()
            }
            Message::Failed(_) => {
                self.state = State::Failed;
                Command::none()
            }
            Message::Reset => {
                self.url = "".to_owned();
                self.state = State::Choosing;
                text_input::focus(Id::new("video_url_input"))
            }
            Message::Empty => Command::none(),
        }
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        // Get the current view based on the state
        let content = match &self.state {
            State::Choosing => self.choosing(),
            State::Preparing => self.downloading("Starting download...", 0.0, 0, 0, 0),
            State::Downloading(progress, total_size, download_speed, eta) => self.downloading(
                "Downloading...",
                *progress,
                *total_size,
                *download_speed,
                *eta,
            ),
            State::Converting => self.downloading("Converting...", 100.0, 0, 0, 0),
            State::Ready => self.ready(),
            State::Failed => self.failed(),
        };

        // Show it
        container(content)
            .padding(Padding::from([0, 32]))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        match &self.state {
            State::Preparing => subscription::run("download-subscription", parsed_output_stream()),
            State::Downloading(_, _, _, _) => {
                subscription::run("download-subscription", parsed_output_stream())
            }
            State::Converting => subscription::run("download-subscription", parsed_output_stream()),

            _ => Subscription::none(),
        }
    }
}
