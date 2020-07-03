use input_linux_sys::input_event;
use std::fs::File;
use std::io;
use std::io::Read;
use std::mem::{size_of, transmute};

pub struct InputDevice {
    file: File,
}

impl InputDevice {
    pub fn new(event_id: u8) -> io::Result<Self> {
        let path = format!("/dev/input/event{}", event_id);
        let file = File::open(&path)?;
        Ok(Self { file })
    }
}

impl Iterator for InputDevice {
    type Item = input_event;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0; size_of::<input_event>()];
        let n = self.file.read(&mut buf).unwrap();
        if n != size_of::<input_event>() {
            None
        } else {
            Some(unsafe { transmute(buf) })
        }
    }
}
