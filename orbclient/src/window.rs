use iced_native::{command::{self, Command}, window};
use std::marker::PhantomData;

pub struct Compositor<Theme> {
    phantom: PhantomData<Theme>,
}

pub fn move_to<Message>(x: i32, y: i32) -> Command<Message> {
    Command::single(command::Action::Window(window::Action::Move { x, y }))
}

pub fn resize<Message>(width: u32, height: u32) -> Command<Message> {
    Command::single(command::Action::Window(window::Action::Resize {
        width,
        height,
    }))
}
