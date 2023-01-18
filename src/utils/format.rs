/// The different download formats, currently on supporting Mp3 and Mp4
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub enum Format {
    #[default]
    Mp3,
    Mp4,
}

impl Format {
    pub const ALL: [Format; 2] = [Format::Mp3, Format::Mp4];
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Format::Mp3 => "Mp3",
                Format::Mp4 => "Mp4",
            }
        )
    }
}
