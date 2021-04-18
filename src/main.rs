mod input_device;
mod virtual_device;

use input_device::InputDevice;
use virtual_device::VirtualKeyboard;

use input_linux::EventKind;
use input_linux::Key;
use input_linux::Key::Reserved as Disabled;
use input_linux::Key::{CapsLock, Grave, LeftAlt, LeftShift, Space, Tab};
use input_linux::Key::{Down, Left, Right, Up};
use input_linux::Key::{Kp0, Kp6, Kp7, Kp8, Kp9};
use input_linux::Key::{Kp1, Kp2, Kp3, Kp4, Kp5};
use input_linux::Key::{Num0, Num6, Num7, Num8, Num9};
use input_linux::Key::{Num1, Num2, Num3, Num4, Num5};
use input_linux::Key::{A, C, D, E, F, Q, R, S, V, W, X, Z};
use input_linux::KeyEvent;

#[rustfmt::skip]
static KEY_MAP: [Key; 26] = [
    Kp1,   Kp2,   Kp3,   Kp4,   Kp5,    // top row
    Kp6,   Kp7,   Kp8,   Kp9,   Kp0,    // second row
    Num1, Num2, Num3, Num4, Num5,       // third row
    Num6, Num7, Num8, Num9, Num0,       // bottom row
    Up,                                 // side button
    LeftShift,                          // up
    Disabled,                           // right
    Down,                               // down
    LeftAlt,                            // left
    Space,                              // space bar
];

fn main() {
    const INPUT_DEVICE_PATH: &str = "/dev/input/by-id/usb-Razer_Razer_Orbweaver_Chroma-event-kbd";
    println!("Listening for keyboard events on {}", INPUT_DEVICE_PATH);

    let virtual_keyboard = VirtualKeyboard::new("Orbweaver Remapper".to_string()).unwrap();
    let orbweaver = InputDevice::new(INPUT_DEVICE_PATH).unwrap();

    for input_event in orbweaver {
        if input_event.kind != EventKind::Key {
            continue;
        }

        let key_event = unsafe { KeyEvent::from_event(&input_event) }.clone();

        match remap(key_event) {
            None => {
                eprintln!("Couldn't find key code {:?}", key_event.key);
                continue;
            }
            Some(key_event) => virtual_keyboard.emit(key_event).unwrap(),
        }
    }
}

fn remap(mut key_event: KeyEvent) -> Option<KeyEvent> {
    #[rustfmt::skip]
    static EVENT_CODES: [Key; 26] = [
        Grave, Num1,  Num2,  Num3,  Num4,   // top row
        Tab, Q, W, E, R,                    // second row
        CapsLock, A, S, D, F,               // third row
        LeftShift, Z, X, C, V,              // bottom row
        LeftAlt,                            // side button
        Up,                                 // up
        Right,                              // right
        Down,                               // down
        Left,                               // left
        Space,                              // space bar
    ];

    let index = EVENT_CODES.iter().position(|&key| key == key_event.key)?;
    key_event.key = KEY_MAP[index];
    Some(key_event)
}
