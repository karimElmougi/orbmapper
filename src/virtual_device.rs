use anyhow::{Context, Result};
use input_linux::uinput::UInputHandle;
use input_linux::Key::Reserved as Disabled;
use input_linux::{EventKind, InputId, Key, KeyEvent};
use input_linux_sys::{input_event, timeval};
use input_linux_sys::{BUS_USB, EV_SYN};

use std::fs::File;

pub struct VirtualKeyboard(UInputHandle<File>);

impl VirtualKeyboard {
    pub fn new(input_device_name: String) -> Result<Self> {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .open("/dev/uinput")
            .context("Couldn't open `/dev/uinput`")?;
        let handle = UInputHandle::new(file);

        enable_new_device(&handle).context("Couldn't enable device")?;

        let id = InputId {
            bustype: BUS_USB,
            vendor: 0x1532,
            product: 0x0207,
            version: 1,
        };

        let name_buffer = input_device_name
            .bytes()
            .take(libc::UINPUT_MAX_NAME_SIZE - 1)
            .collect::<Vec<_>>();

        let ff_effects_max = 0;

        handle
            .create(&id, &name_buffer, ff_effects_max, &[])
            .context("Couldn't create virtual device")?;

        Ok(Self(handle))
    }

    pub fn emit(&self, event: KeyEvent) -> Result<()> {
        if matches!(event.key, Disabled) {
            return Ok(());
        }

        self.0.write(&[*event.as_event().as_raw()])?;
        self.sync_event()?;

        Ok(())
    }

    fn sync_event(&self) -> Result<()> {
        let sync_event = input_event {
            type_: EV_SYN as u16,
            time: timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
            code: 0,
            value: 0,
        };

        self.0
            .write(&[sync_event])
            .context("Couldn't write SYN event")?;

        Ok(())
    }
}

fn enable_new_device(handle: &UInputHandle<File>) -> Result<()> {
    handle
        .set_evbit(EventKind::Key)
        .context("Couldn't set EV bit for KEY")?;

    for key in Key::iter() {
        handle
            .set_keybit(key)
            .with_context(|| format!("Couldn't set key bit for `{:?}`", key))?;
    }

    handle
        .set_evbit(EventKind::Synchronize)
        .context("Couldn't set EV bit for SYN")?;

    Ok(())
}
