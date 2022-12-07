pub use iced_graphics::Antialiasing;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompositorSettings {
    pub default_font: Option<&'static [u8]>,
    pub default_text_size: u16,
    pub text_multithreading: bool,
    pub antialiasing: Option<Antialiasing>,
}

impl Default for CompositorSettings {
    fn default() -> Self {
        Self {
            default_font: None,
            default_text_size: 20,
            text_multithreading: false,
            antialiasing: None,
        }
    }
}

impl CompositorSettings {
    pub fn from_env() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, Default)]
pub struct RuntimeSettings<Flags> {
    //TODO: add more settings
    pub flags: Flags,
}
