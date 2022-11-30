use crate::{Renderer, Settings};

use core::marker::PhantomData;

/// A window graphics backend for iced powered by `orbclient`.
pub struct Compositor<Theme> {
    theme: PhantomData<Theme>,
}

impl<Theme> iced_graphics::window::Compositor for Compositor<Theme> {
    type Settings = Settings<()>;
    type Renderer = Renderer<Theme>;
    type Surface = ();
}
