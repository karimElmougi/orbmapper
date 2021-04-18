use libc::input_event;
use std::fs::File;
use std::io;
use std::io::Read;
use std::mem::{size_of, transmute};

pub struct InputDevice(File);

impl InputDevice {
    pub fn new(path: &str) -> io::Result<Self> {
        let file = File::open(path)?;
        Ok(Self(file))
    }
}

impl Iterator for InputDevice {
    type Item = input_event;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0; size_of::<input_event>()];
        let n = self.0.read(&mut buf).unwrap();
        if n != size_of::<input_event>() {
            None
        } else {
            Some(unsafe { transmute(buf) })
        }
    }
}
