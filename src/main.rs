use input_linux_sys::{
    input_event, input_id, timeval, ui_dev_create, ui_dev_setup, ui_set_evbit, ui_set_keybit,
    uinput_setup,
};
use input_linux_sys::{BUS_USB, EV_KEY, EV_SYN};
use libc::{c_void, open, read, write, O_NONBLOCK, O_RDWR, O_WRONLY};
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::mem::size_of;

const KEY_NUMPAD_1: u16 = input_linux_sys::KEY_KP1 as u16;
const KEY_NUMPAD_2: u16 = input_linux_sys::KEY_KP2 as u16;
const KEY_NUMPAD_3: u16 = input_linux_sys::KEY_KP3 as u16;
const KEY_NUMPAD_4: u16 = input_linux_sys::KEY_KP4 as u16;
const KEY_NUMPAD_5: u16 = input_linux_sys::KEY_KP5 as u16;
const KEY_NUMPAD_6: u16 = input_linux_sys::KEY_KP6 as u16;
const KEY_NUMPAD_7: u16 = input_linux_sys::KEY_KP7 as u16;
const KEY_NUMPAD_8: u16 = input_linux_sys::KEY_KP8 as u16;
const KEY_NUMPAD_9: u16 = input_linux_sys::KEY_KP9 as u16;
const KEY_NUMPAD_0: u16 = input_linux_sys::KEY_KP0 as u16;

const KEY_1: u16 = input_linux_sys::KEY_1 as u16;
const KEY_2: u16 = input_linux_sys::KEY_2 as u16;
const KEY_3: u16 = input_linux_sys::KEY_3 as u16;
const KEY_4: u16 = input_linux_sys::KEY_4 as u16;
const KEY_5: u16 = input_linux_sys::KEY_5 as u16;
const KEY_6: u16 = input_linux_sys::KEY_6 as u16;
const KEY_7: u16 = input_linux_sys::KEY_7 as u16;
const KEY_8: u16 = input_linux_sys::KEY_8 as u16;
const KEY_9: u16 = input_linux_sys::KEY_9 as u16;
const KEY_0: u16 = input_linux_sys::KEY_0 as u16;

const KEY_UP: u16 = input_linux_sys::KEY_UP as u16;
const KEY_DOWN: u16 = input_linux_sys::KEY_DOWN as u16;
const KEY_ALT: u16 = input_linux_sys::KEY_LEFTALT as u16;
const KEY_SHIFT: u16 = input_linux_sys::KEY_LEFTSHIFT as u16;
const KEY_SPACE: u16 = input_linux_sys::KEY_SPACE as u16;
const KEY_DISABLED: u16 = 0;

static KEY_MAP: [u16; 26] = [
    KEY_NUMPAD_1, // top row begin
    KEY_NUMPAD_2,
    KEY_NUMPAD_3,
    KEY_NUMPAD_4,
    KEY_NUMPAD_5, // top row end
    KEY_NUMPAD_6, // second row begin
    KEY_NUMPAD_7,
    KEY_NUMPAD_8,
    KEY_NUMPAD_9,
    KEY_NUMPAD_0, // second row end
    KEY_1,        // third row begin
    KEY_2,
    KEY_3,
    KEY_4,
    KEY_5, // third row end
    KEY_6, // bottom row begin
    KEY_7,
    KEY_8,
    KEY_9,
    KEY_0,        // bottom row end
    KEY_UP,       // side button
    KEY_SHIFT,    // up
    KEY_DISABLED, // right
    KEY_DOWN,     // down
    KEY_ALT,      // left
    KEY_SPACE,    // space bar
];

struct FileDescriptor {
    raw: i32,
}

impl From<i32> for FileDescriptor {
    fn from(i: i32) -> Self {
        FileDescriptor { raw: i }
    }
}

const DEFAULT_INPUT_EVENT: input_event = input_event {
    time: timeval {
        tv_sec: 0,
        tv_usec: 0,
    },
    type_: 0,
    code: 0,
    value: 0,
};

fn main() {
    let event_id =
        find_orbweaver_event_id().expect("Couldn't detect any connected Razer Orbweaver");

    println!("Listening on /dev/input/event{}", event_id);
    let keyboard_fd = setup_uinput_device();
    let orbweaver_fd = open_orbweaver_event_device(event_id);

    loop {
        let mut events = [DEFAULT_INPUT_EVENT; 2];

        let ptr = &mut events[0] as *mut input_event;
        unsafe {
            read(
                orbweaver_fd.raw,
                ptr as *mut c_void,
                size_of::<input_event>() * events.len(),
            );
        }

        let key_event = {
            let mut key_event = None;
            for event in &events {
                if event.type_ == EV_KEY as u16 {
                    key_event = Some(event);
                    break;
                }
            }
            key_event
        };

        if key_event.is_none() {
            continue;
        }

        let key_event = key_event.unwrap();

        let keypad_code = event_code_to_keypad_code(key_event.code);
        if keypad_code.is_none() {
            println!("Couldn't find key code {}", key_event.code);
            continue;
        }

        let index = keypad_code.unwrap();

        let generated_key_event = input_event {
            code: KEY_MAP[index],
            type_: EV_KEY as u16,
            ..*key_event
        };

        if generated_key_event.code == KEY_DISABLED {
            continue;
        }

        unsafe {
            let mut return_code = write(
                keyboard_fd.raw,
                &generated_key_event as *const input_event as *const c_void,
                size_of::<input_event>(),
            );
            if return_code < 0 {
                panic!("Couldn't write key event to emulated file descriptor");
            }

            let sync_event = input_event {
                type_: EV_SYN as u16,
                ..DEFAULT_INPUT_EVENT
            };
            return_code = write(
                keyboard_fd.raw,
                &sync_event as *const input_event as *const c_void,
                size_of::<input_event>(),
            );

            if return_code < 0 {
                panic!("Couldn't write sync event to emulated file descriptor");
            }
        }
    }
}

fn event_code_to_keypad_code(event_code: u16) -> Option<usize> {
    static EVENT_CODES: [u16; 26] = [
        41, 2, 3, 4, 5, 15, 16, 17, 18, 19, 58, 30, 31, 32, 33, 42, 44, 45, 46, 47, 56, 103, 106,
        108, 105, 57,
    ];

    EVENT_CODES.iter().position(|&code| code == event_code)
}

fn find_orbweaver_event_id() -> Option<u8> {
    let mut devices_file = File::open("/proc/bus/input/devices").unwrap();
    let mut content = String::new();
    let _ = devices_file.read_to_string(&mut content);

    for device in content
        .split("\n\n")
        .filter(|d| d.contains("Vendor=1532 Product=0207 Version=0111"))
    {
        const HANDLERS_HEADER: &str = "H: Handlers=sysrq kbd event";
        for line in device.split('\n') {
            if line.starts_with(HANDLERS_HEADER) && !line.contains("mouse") && line.contains("leds")
            {
                let id = &line[HANDLERS_HEADER.len()..]
                    .split_whitespace()
                    .next()
                    .unwrap();
                let id = id.parse::<u8>().unwrap();
                return Some(id);
            }
        }
    }

    None
}

fn setup_uinput_device() -> FileDescriptor {
    let name = {
        let mut name = [0; 80];
        for (pos, &c) in name.iter_mut().zip(b"orbweaver key remapper".iter()){
            *pos = c as i8;
        }
        name
    };

    let keyboard_emulator = uinput_setup {
        name,
        id: input_id {
            bustype: BUS_USB,
            vendor: 0x1532,
            product: 0x0207,
            version: 1,
        },
        ff_effects_max: 0,
    };

    unsafe {
        let path = CString::new("/dev/uinput").unwrap();
        let input_file_descriptor = open(path.as_ptr(), O_WRONLY | O_NONBLOCK);
        if input_file_descriptor < 0 {
            panic!("Failed to open /dev/uinput");
        }

        ui_set_evbit(input_file_descriptor, EV_KEY).expect("Failed to UI_SET_EVBIT");

        for event in 0..255 {
            if ui_set_keybit(input_file_descriptor, event).is_err() {
                println!("Failed to UI_SET_EVBIT for event {}", event);
            }
        }

        ui_set_evbit(input_file_descriptor, EV_SYN).expect("Failed to UI_SET_EVBIT");
        ui_dev_setup(
            input_file_descriptor,
            &keyboard_emulator as *const uinput_setup,
        )
        .expect("Failed to write to input file descriptor");
        ui_dev_create(input_file_descriptor).expect("Failed to UI_DEV_CREATE");

        input_file_descriptor.into()
    }
}

fn open_orbweaver_event_device(orbweaver_event_id: u8) -> FileDescriptor {
    let path = CString::new(format!("/dev/input/event{}", orbweaver_event_id)).unwrap();
    unsafe { open(path.as_ptr(), O_RDWR).into() }
}
