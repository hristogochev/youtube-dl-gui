use iced::window::Icon;

pub static ICON: &[u8] = include_bytes!("../../assets/images/icon.ico");
pub const ICON_HEIGHT: u32 = 256;
pub const ICON_WIDTH: u32 = 256;

pub fn get_icon() -> Result<Icon, String> {
    image::load_from_memory(ICON)
        .map_err(|err| err.to_string())
        .and_then(|image| {
            Icon::from_rgba(image.as_bytes().to_vec(), ICON_HEIGHT, ICON_WIDTH)
                .map_err(|err| err.to_string())
        })
        .map_err(|err| format!("Could not load application icon: {err}"))
}
