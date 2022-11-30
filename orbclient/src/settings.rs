//! Configure a renderer.
pub use iced_graphics::Antialiasing;

/// The settings of a [`Backend`].
///
/// [`Backend`]: crate::Backend
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Settings<Flags> {
    //TODO: split out compositor settings
    /// The bytes of the font that will be used by default.
    ///
    /// If `None` is provided, a default system font will be chosen.
    pub default_font: Option<&'static [u8]>,

    /// The default size of text.
    ///
    /// By default, it will be set to 20.
    pub default_text_size: u16,

    /// If enabled, spread text workload in multiple threads when multiple cores
    /// are available.
    ///
    /// By default, it is disabled.
    pub text_multithreading: bool,

    /// The antialiasing strategy that will be used for triangle primitives.
    ///
    /// By default, it is `None`.
    pub antialiasing: Option<Antialiasing>,

    //TODO: more settings from iced_winit
    pub flags: Flags,
}

impl<Flags: Default> Default for Settings<Flags> {
    fn default() -> Self {
        Self {
            default_font: None,
            default_text_size: 20,
            text_multithreading: false,
            antialiasing: None,
            flags: Flags::default(),
        }
    }
}

impl<Flags: Default> Settings<Flags> {
    /// Creates new [`Settings`] using environment configuration.
    ///
    /// Currently, this is equivalent to calling [`Settings::default`].
    pub fn from_env() -> Self {
        Self::default()
    }
}
