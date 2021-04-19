mod input_device;
mod virtual_device;

use input_device::InputDevice;
use virtual_device::VirtualKeyboard;

use anyhow::{Context, Result};
use arrayvec::ArrayVec;
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
use serde_derive::{Deserialize, Serialize};
use structopt::StructOpt;

use std::path::PathBuf;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    config_file: Option<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize)]
struct KeyMapConfig {
    top_row: [Key; 5],
    second_row: [Key; 5],
    third_row: [Key; 5],
    bottom_row: [Key; 5],
    up: Key,
    down: Key,
    left: Key,
    right: Key,
    side_button: Key,
    space_bar: Key,
}

impl KeyMapConfig {
    fn flatten(self) -> ArrayVec<Key, 26> {
        let mut key_map = [
            self.top_row,
            self.second_row,
            self.third_row,
            self.bottom_row,
        ]
        .concat()
        .iter()
        .copied()
        .collect::<ArrayVec<_, 26>>();

        key_map.push(self.side_button);
        key_map.push(self.up);
        key_map.push(self.right);
        key_map.push(self.down);
        key_map.push(self.left);
        key_map.push(self.space_bar);

        key_map
    }
}

impl Default for KeyMapConfig {
    fn default() -> Self {
        Self {
            top_row: [Kp1, Kp2, Kp3, Kp4, Kp5],
            second_row: [Kp6, Kp7, Kp8, Kp9, Kp0],
            third_row: [Num1, Num2, Num3, Num4, Num5],
            bottom_row: [Num6, Num7, Num8, Num9, Num0],
            side_button: Up,
            up: LeftShift,
            right: Disabled,
            down: Down,
            left: LeftAlt,
            space_bar: Space,
        }
    }
}

fn main() -> Result<()> {
    sudo::escalate_if_needed().unwrap();

    let opt = Opt::from_args();

    let key_map = opt
        .config_file
        .map(load_config)
        .transpose()?
        .unwrap_or_default()
        .flatten();

    run(&key_map)
}

fn load_config(path: PathBuf) -> Result<KeyMapConfig> {
    let path = path
        .canonicalize()
        .with_context(|| format!("Invalid path {:?}", path))?;

    let s =
        std::fs::read_to_string(&path).with_context(|| format!("Couldn't read from {:?}", path))?;

    toml::from_str(&s).with_context(|| format!("Invalid TOML at {:?}", path))
}

fn run(key_map: &ArrayVec<Key, 26>) -> Result<()> {
    const INPUT_DEVICE_PATH: &str = "/dev/input/by-id/usb-Razer_Razer_Orbweaver_Chroma-event-kbd";
    println!("Listening for keyboard events on {}", INPUT_DEVICE_PATH);

    let virtual_keyboard = VirtualKeyboard::new("Orbweaver Remapper".to_string())?;
    let orbweaver = InputDevice::new(INPUT_DEVICE_PATH)?;

    for input_event in orbweaver {
        if input_event.kind != EventKind::Key {
            continue;
        }

        let key_event = *unsafe { KeyEvent::from_event(&input_event) };

        match remap(key_map, key_event) {
            None => {
                eprintln!("Couldn't find key code {:?}", key_event.key);
                continue;
            }
            Some(key_event) => virtual_keyboard.emit(key_event)?,
        }
    }

    Ok(())
}

fn remap(key_map: &ArrayVec<Key, 26>, mut key_event: KeyEvent) -> Option<KeyEvent> {
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
    key_event.key = key_map[index];
    Some(key_event)
}
