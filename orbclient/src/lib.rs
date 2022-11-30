#[doc(no_inline)]
pub use iced_native::*;

#[cfg(feature = "application")]
pub mod application;

#[cfg(feature = "application")]
pub use application::Application;

mod backend;
pub use backend::Backend;

pub mod clipboard;
pub use clipboard::Clipboard;

mod error;
pub use error::Error;

pub mod settings;
pub use settings::Settings;

pub mod window;

/// An [`orbclient`] graphics renderer for [`iced`].
pub type Renderer<Theme = iced_native::Theme> =
    iced_graphics::Renderer<Backend, Theme>;
