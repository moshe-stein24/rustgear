use crate::input::{InputDevice, InputDeviceKind};

#[derive(Debug, Clone, Default)]
pub struct InputDeviceManager {
    pub devices: Vec<InputDevice>,
}

impl InputDeviceManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enumerate(&mut self) {
        self.devices.clear();
        self.devices.push(InputDevice::new(0, InputDeviceKind::Keyboard, "Keyboard"));
        self.devices.push(InputDevice::new(1, InputDeviceKind::Mouse, "Mouse"));
        self.devices.push(InputDevice::new(2, InputDeviceKind::Joystick, "Joystick"));
    }

    pub fn find(&self, kind: InputDeviceKind) -> Option<&InputDevice> {
        self.devices.iter().find(|d| d.kind == kind)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn enumerate_creates_devices() {
        let mut mgr = InputDeviceManager::new();
        mgr.enumerate();
        assert!(mgr.find(InputDeviceKind::Keyboard).is_some());
        assert!(mgr.find(InputDeviceKind::Joystick).is_some());
    }
}
