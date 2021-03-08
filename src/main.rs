mod input_device;
mod keys;
mod virtual_device;

use input_device::InputDevice;
use keys::Key;
use virtual_device::VirtualKeyboard;

const KEY_PRESS_EVENT: u16 = input_linux_sys::EV_KEY as u16;

#[rustfmt::skip]
static KEY_MAP: [Key; 26] = [
    Key::Numpad1,   Key::Numpad2,   Key::Numpad3,   Key::Numpad4,   Key::Numpad5,   // top row
    Key::Numpad6,   Key::Numpad7,   Key::Numpad8,   Key::Numpad9,   Key::Numpad0,   // second row
    Key::Keyboard1, Key::Keyboard2, Key::Keyboard3, Key::Keyboard4, Key::Keyboard5, // third row
    Key::Keyboard6, Key::Keyboard7, Key::Keyboard8, Key::Keyboard9, Key::Keyboard0, // bottom row
    Key::Up,        // side button
    Key::LeftShit,  // up
    Key::Disabled,  // right
    Key::Down,      // down
    Key::LeftAlt,   // left
    Key::Space,     // space bar
];

fn main() {
    const INPUT_DEVICE_PATH: &str = "/dev/input/by-id/usb-Razer_Razer_Orbweaver_Chroma-event-kbd";
    println!("Listening for keyboard events on {}", INPUT_DEVICE_PATH);

    let virtual_keyboard = VirtualKeyboard::new("Orbweaver Remapper".to_string()).unwrap();
    let orbweaver = InputDevice::new(INPUT_DEVICE_PATH).unwrap();

    for key_event in orbweaver {
        if key_event.type_ != KEY_PRESS_EVENT {
            continue;
        }

        match event_code_to_key(key_event.code) {
            None => {
                println!("Couldn't find key code {}", key_event.code);
                continue;
            }
            Some(key) => virtual_keyboard.press(key, key_event.value).unwrap(),
        }
    }
}

fn event_code_to_key(event_code: u16) -> Option<Key> {
    #[rustfmt::skip]
    static EVENT_CODES: [u16; 26] = [
        41, 2,  3,  4,  5,     // top row
        15, 16, 17, 18, 19, // second row
        58, 30, 31, 32, 33, // third row
        42, 44, 45, 46, 47, // bottom row
        56,                 // side button
        103,                // up
        106,                // right
        108,                // down
        105,                // left
        57,                 // space bar
    ];

    let index = EVENT_CODES.iter().position(|&code| code == event_code)?;
    Some(KEY_MAP[index])
}
