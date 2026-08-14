use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::time::Instant;
use std::io::Read;

use crate::input::{InputState, InputAxis, InputEvent, InputBindingMap};

#[derive(Debug, Clone, Copy, Default)]
pub struct JoystickState {
    pub roll: f64,
    pub pitch: f64,
    pub yaw: f64,
    pub throttle: f64,
}

impl JoystickState {
    pub fn apply(&self, input: &mut InputState, bindings: &InputBindingMap) {
        let events = [
            InputEvent { axis: InputAxis::Roll, raw: self.roll },
            InputEvent { axis: InputAxis::Pitch, raw: self.pitch },
            InputEvent { axis: InputAxis::Yaw, raw: self.yaw },
            InputEvent { axis: InputAxis::Throttle, raw: self.throttle },
        ];
        crate::input::apply_events(input, &events, bindings);
    }
}

#[repr(C, packed)]
struct JsEvent {
    time: u32,
    value: i16,
    type_: u8,
    number: u8,
}

const JS_EVENT_AXIS: u8 = 0x02;

#[derive(Debug)]
pub struct JoystickInput {
    file: Option<File>,
    axes: [f64; 8],
    pub state: JoystickState,
    last_poll: Instant,
    has_fresh_data: bool,
}

impl JoystickInput {
    pub fn new() -> Self {
        Self {
            file: File::open("/dev/input/js0").ok(),
            axes: [0.0; 8],
            state: JoystickState::default(),
            last_poll: Instant::now(),
            has_fresh_data: false,
        }
    }

    fn deadzone(v: f64) -> f64 {
        if v.abs() < 0.1 { 0.0 } else { v }
    }

    pub fn has_fresh_data(&self) -> bool {
        self.has_fresh_data
    }

    pub fn clear_fresh_data(&mut self) {
        self.has_fresh_data = false;
    }

    pub fn poll(&mut self) {
        let Some(file) = self.file.as_mut() else { return };

        if self.last_poll.elapsed() < std::time::Duration::from_millis(8) {
            return;
        }
        self.last_poll = Instant::now();

        // Make reads non-blocking so this never stalls the GUI/event loop.
        unsafe {
            let fd = file.as_raw_fd();
            let mut flags = libc::fcntl(fd, libc::F_GETFL);
            if flags != -1 {
                flags |= libc::O_NONBLOCK;
                libc::fcntl(fd, libc::F_SETFL, flags);
            }
        }

        let mut buf = [0u8; 8];
        let mut changed = false;

        loop {
            match file.read_exact(&mut buf) {
                Ok(_) => {
                    let ev = unsafe { std::ptr::read_unaligned(buf.as_ptr() as *const JsEvent) };
                    if ev.type_ & JS_EVENT_AXIS != 0 && (ev.number as usize) < self.axes.len() {
                        let normalized = (ev.value as f64) / 32767.0;
                        self.axes[ev.number as usize] = normalized.clamp(-1.0, 1.0);
                        changed = true;
                    }
                }
                Err(_) => break,
            }
        }

        if !changed {
            return;
        }

        // Left stick: axes 0,1 -> roll, pitch
        // Right stick: axes 2,3 -> yaw, unused
        // Triggers: axes 4,5 -> L2, R2 throttle
        let roll = Self::deadzone(self.axes[0]);
        let pitch = Self::deadzone(-self.axes[1]); // invert: down = nose up
        let yaw = Self::deadzone(self.axes[2]);
        let throttle = (Self::deadzone(self.axes[5]) - Self::deadzone(self.axes[4])).clamp(-1.0, 1.0);

        let state = JoystickState { roll, pitch, yaw, throttle };
        if roll != 0.0 || pitch != 0.0 || yaw != 0.0 || throttle != 0.0 {
            self.state = state;
            self.has_fresh_data = true;
        }
    }
}
