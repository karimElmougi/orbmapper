mod input_device;
mod keys;
mod virtual_device;

use input_device::InputDevice;
use keys::Key;
use virtual_device::VirtualKeyboard;

use input_linux_sys::EV_KEY;

static KEY_MAP: [Key; 26] = [
    Key::Numpad1, // top row begin
    Key::Numpad2,
    Key::Numpad3,
    Key::Numpad4,
    Key::Numpad5, // top row end
    Key::Numpad6, // second row begin
    Key::Numpad7,
    Key::Numpad8,
    Key::Numpad9,
    Key::Numpad0,   // second row end
    Key::Keyboard1, // third row begin
    Key::Keyboard2,
    Key::Keyboard3,
    Key::Keyboard4,
    Key::Keyboard5, // third row end
    Key::Keyboard6, // bottom row begin
    Key::Keyboard7,
    Key::Keyboard8,
    Key::Keyboard9,
    Key::Keyboard0, // bottom row end
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
        if key_event.type_ != EV_KEY as u16 {
            continue;
        }

        let keypad_code = event_code_to_keypad_code(key_event.code);
        if keypad_code.is_none() {
            println!("Couldn't find key code {}", key_event.code);
            continue;
        }

        let keypad_code = keypad_code.unwrap();
        virtual_keyboard
            .press(KEY_MAP[keypad_code], key_event.value)
            .unwrap();
    }
}

fn event_code_to_keypad_code(event_code: u16) -> Option<usize> {
    static EVENT_CODES: [u16; 26] = [
        41, 2, 3, 4, 5, 15, 16, 17, 18, 19, 58, 30, 31, 32, 33, 42, 44, 45, 46, 47, 56, 103, 106,
        108, 105, 57,
    ];

    EVENT_CODES.iter().position(|&code| code == event_code)
}
