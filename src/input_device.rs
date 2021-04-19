use anyhow::{Context, Result};
use input_linux::evdev::EvdevHandle;
use input_linux::InputEvent;
use libc::{input_event, timeval};

use std::fs::File;

const BUFFER_SIZE: usize = 10;
pub struct InputDevice {
    handle: EvdevHandle<File>,
    buffer: [input_event; BUFFER_SIZE],
    buffer_size: usize,
    index: usize,
}

impl InputDevice {
    pub fn new(path: &str) -> Result<Self> {
        let input_device_file =
            File::open(path).with_context(|| format!("Couldn't open input device `{}`", path))?;

        let handle = EvdevHandle::new(input_device_file);
        handle.grab(true).context("Couldn't grab input device")?;

        let default_event = input_event {
            time: timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
            type_: 0,
            code: 0,
            value: 0,
        };

        Ok(Self {
            handle,
            buffer: [default_event; BUFFER_SIZE],
            buffer_size: 0,
            index: 0,
        })
    }
}

impl Iterator for InputDevice {
    type Item = InputEvent;

    fn next(&mut self) -> Option<Self::Item> {
        while self.buffer_size == 0 {
            self.buffer_size = self.handle.read(&mut self.buffer).unwrap();
            self.index = 0;
        }

        let event = self.buffer.get(self.index).unwrap();
        self.index += 1;
        self.buffer_size -= 1;

        Some(*InputEvent::from_raw(event).unwrap())
    }
}
