use crate::keys::Key;
use fcntl::OFlag;
use input_linux_sys::{input_event, input_id, timeval};
use input_linux_sys::{ui_dev_create, ui_dev_setup, ui_set_evbit, ui_set_keybit, uinput_setup};
use input_linux_sys::{BUS_USB, EV_KEY, EV_SYN};
use nix::fcntl;
use nix::sys::stat;
use nix::unistd;
use std::{convert::TryInto, mem::size_of};
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
    pub fn new(input_device_name: String) -> input_linux_sys::Result<Self> {
        let name_buffer = {
            let mut buf = [0; 80];
            for (pos, c) in buf.iter_mut().zip(input_device_name.bytes()) {
                *pos = c as i8;
            }
            buf
        };

        let new_device = uinput_setup {
            name: name_buffer,
            id: input_id {
                bustype: BUS_USB,
                vendor: 0x1532,
                product: 0x0207,
                version: 1,
            },
            ff_effects_max: 0,
        };

        let uinput_file_descriptor = fcntl::open(
            "/dev/uinput",
            OFlag::O_WRONLY | OFlag::O_NONBLOCK,
            stat::Mode::empty(),
        )?
        .into();

        enable_new_device(uinput_file_descriptor)?;
        create_new_device(uinput_file_descriptor, new_device)?;

        Ok(Self {
            fd: uinput_file_descriptor,
        })
    }

    pub fn press(self, key: Key, value: i32) -> Result<(), nix::Error> {
        if key.is_disabled() {
            return Ok(());
        }

        write_event(self.fd, key.input_event(value))?;
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

        write_event(self.fd, sync_event)?;

        Ok(())
    }
}

fn write_event(fd: FileDescriptor, event: input_event) -> Result<(), nix::Error> {
    let ptr = &event as *const _ as *const u8;
    let size = size_of::<input_event>();
    unsafe { unistd::write(fd.raw, slice::from_raw_parts(ptr, size))? };
    Ok(())
}

fn enable_new_device(uinput_fd: FileDescriptor) -> input_linux_sys::Result<()> {
    unsafe {
        ui_set_evbit(uinput_fd.raw, EV_KEY.try_into().unwrap())?;
        for event in 0..255 {
            if ui_set_keybit(uinput_fd.raw, event).is_err() {
                println!("Failed to UI_SET_EVBIT for event {}", event);
            }
        }
        ui_set_evbit(uinput_fd.raw, EV_SYN.try_into().unwrap())?;
    }
    Ok(())
}

fn create_new_device(
    uinput_fd: FileDescriptor,
    device: uinput_setup,
) -> input_linux_sys::Result<()> {
    unsafe {
        ui_dev_setup(uinput_fd.raw, &device as *const _)?;
        ui_dev_create(uinput_fd.raw)?;
    }
    Ok(())
}
