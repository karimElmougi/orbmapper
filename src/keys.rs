use input_linux_sys::{KEY_0, KEY_1, KEY_2, KEY_3, KEY_4, KEY_5, KEY_6, KEY_7, KEY_8, KEY_9};
use input_linux_sys::{
    KEY_DOWN, KEY_LEFT, KEY_LEFTALT, KEY_LEFTSHIFT, KEY_RESERVED, KEY_RIGHT, KEY_SPACE, KEY_UP,
};
use input_linux_sys::{
    KEY_KP0, KEY_KP1, KEY_KP2, KEY_KP3, KEY_KP4, KEY_KP5, KEY_KP6, KEY_KP7, KEY_KP8, KEY_KP9,
};

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq)]
pub enum Key {
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    Numpad0,
    Keyboard1,
    Keyboard2,
    Keyboard3,
    Keyboard4,
    Keyboard5,
    Keyboard6,
    Keyboard7,
    Keyboard8,
    Keyboard9,
    Keyboard0,
    Up,
    Down,
    Left,
    Right,
    LeftAlt,
    LeftShit,
    Space,
    Disabled,
}

impl Key {
    pub fn code(self) -> u16 {
        (match self {
            Key::Numpad0 => KEY_KP0,
            Key::Numpad1 => KEY_KP1,
            Key::Numpad2 => KEY_KP2,
            Key::Numpad3 => KEY_KP3,
            Key::Numpad4 => KEY_KP4,
            Key::Numpad5 => KEY_KP5,
            Key::Numpad6 => KEY_KP6,
            Key::Numpad7 => KEY_KP7,
            Key::Numpad8 => KEY_KP8,
            Key::Numpad9 => KEY_KP9,
            Key::Keyboard0 => KEY_0,
            Key::Keyboard1 => KEY_1,
            Key::Keyboard2 => KEY_2,
            Key::Keyboard3 => KEY_3,
            Key::Keyboard4 => KEY_4,
            Key::Keyboard5 => KEY_5,
            Key::Keyboard6 => KEY_6,
            Key::Keyboard7 => KEY_7,
            Key::Keyboard8 => KEY_8,
            Key::Keyboard9 => KEY_9,
            Key::Up => KEY_UP,
            Key::Down => KEY_DOWN,
            Key::Left => KEY_LEFT,
            Key::Right => KEY_RIGHT,
            Key::LeftAlt => KEY_LEFTALT,
            Key::LeftShit => KEY_LEFTSHIFT,
            Key::Space => KEY_SPACE,
            Key::Disabled => KEY_RESERVED,
        }) as u16
    }

    pub fn is_disabled(self) -> bool {
        match self {
            Key::Disabled => true,
            _ => false,
        }
    }
}
