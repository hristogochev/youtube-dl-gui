use iced::Font;

pub static RAW_OPEN_SANS_REGULAR: &[u8] =
    include_bytes!("../../../assets/fonts/OpenSans-Regular.ttf");
pub static RAW_OPEN_SANS_SEMIBOLD: &[u8] =
    include_bytes!("../../../assets/fonts/OpenSans-SemiBold.ttf");
pub static RAW_OPEN_SANS_BOLD: &[u8] = include_bytes!("../../../assets/fonts/OpenSans-Regular.ttf");

pub static OPEN_SANS_REGULAR: Font = Font::External {
    name: "OPEN_SANS_REGULAR",
    bytes: RAW_OPEN_SANS_REGULAR,
};

pub static OPEN_SANS_SEMIBOLD: Font = Font::External {
    name: "OPEN_SANS_SEMIBOLD",
    bytes: RAW_OPEN_SANS_SEMIBOLD,
};

pub static OPEN_SANS_BOLD: Font = Font::External {
    name: "OPEN_SANS_BOLD",
    bytes: RAW_OPEN_SANS_BOLD,
};
