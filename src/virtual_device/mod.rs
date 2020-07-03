use crate::keys::Key;
use fcntl::OFlag;
use input_linux_sys::{input_event, input_id, timeval};
use input_linux_sys::{ui_dev_create, ui_dev_setup, ui_set_evbit, ui_set_keybit, uinput_setup};
use input_linux_sys::{BUS_USB, EV_KEY, EV_SYN};
use nix::fcntl;
use nix::sys::stat;
use nix::unistd;
use std::mem::size_of;
use std::slice;

#[derive(Clone, Copy)]
struct FileDescriptor {
    raw: i32,
}

impl From<i32> for FileDescriptor {
    fn from(fd: i32) -> Self {
        Self { raw: fd }
    }
}

#[derive(Clone, Copy)]
pub struct VirtualKeyboard {
    fd: FileDescriptor,
}

impl VirtualKeyboard {
    pub fn new(name: String) -> input_linux_sys::Result<Self> {
        let path = "/dev/uinput";
        let fd = fcntl::open(
            path,
            OFlag::O_WRONLY | OFlag::O_NONBLOCK,
            stat::Mode::empty(),
        )?;
        let virtual_keyboard = Self { fd: fd.into() };

        let name = {
            let mut buf = [0; 80];
            for (pos, c) in buf.iter_mut().zip(name.bytes()) {
                *pos = c as i8;
            }
            buf
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
            ui_set_evbit(fd, EV_KEY)?;
            for event in 0..255 {
                if ui_set_keybit(fd, event).is_err() {
                    println!("Failed to UI_SET_EVBIT for event {}", event);
                }
            }
            ui_set_evbit(fd, EV_SYN)?;
            ui_dev_setup(fd, &keyboard_emulator as *const _)?;
            ui_dev_create(fd)?;
        }

        Ok(virtual_keyboard)
    }

    pub fn press(self, key: Key, value: i32) -> Result<(), nix::Error> {
        if key.is_disabled() {
            return Ok(());
        }

        let generated_key_event = input_event {
            code: key.code(),
            type_: EV_KEY as u16,
            time: timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
            value,
        };

        unsafe {
            let ptr = &generated_key_event as *const _ as *const u8;
            let size = size_of::<input_event>();

            unistd::write(self.fd.raw, slice::from_raw_parts(ptr, size))?;
        }

        self.sync_event()?;

        Ok(())
    }

    fn sync_event(self) -> Result<(), nix::Error> {
        let sync_event = input_event {
            type_: EV_SYN as u16,
            time: timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
            code: 0,
            value: 0,
        };
        let ptr = &sync_event as *const _ as *const u8;
        let size = size_of::<input_event>();

        unsafe { unistd::write(self.fd.raw, slice::from_raw_parts(ptr, size))? };

        Ok(())
    }
}
