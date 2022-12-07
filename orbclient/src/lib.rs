#[doc(no_inline)]
pub use iced_native::*;

#[cfg(feature = "application")]
pub mod application;

#[cfg(feature = "application")]
pub use application::Application;

pub mod clipboard;
pub use clipboard::Clipboard;

mod error;
pub use error::Error;

mod renderer;

pub mod settings;
pub use settings::CompositorSettings as Settings;

pub mod window;

/// An [`orbclient`] graphics renderer for [`iced`].
pub type Renderer<Theme = iced_native::Theme> =
    renderer::Renderer<Theme>;
