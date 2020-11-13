use raw_window_handle::RawWindowHandle;

#[cfg(target_os = "windows")]
mod win;
#[cfg(target_os = "windows")]
pub use win::*;

#[cfg(target_os = "linux")]
mod x11;
#[cfg(target_os = "linux")]
pub use crate::x11::*;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::*;

mod event;
mod keyboard;
mod mouse_cursor;
mod window_info;
mod window_open_options;
pub use event::*;
pub use keyboard::KeyEvent;
pub use mouse_cursor::MouseCursor;
pub use window_info::*;
pub use window_open_options::*;

#[derive(Debug)]
pub enum Parent {
    None,
    AsIfParented,
    WithParent(RawWindowHandle),
}

unsafe impl Send for Parent {}

pub trait WindowHandler {
    type Message;

    fn on_frame(&mut self);
    fn on_event(&mut self, window: &mut Window, event: Event);
    fn on_message(&mut self, window: &mut Window, message: Self::Message);
}
