#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputDeviceKind {
    Keyboard,
    Mouse,
    Joystick,
}

#[derive(Debug, Clone)]
pub struct InputDevice {
    pub id: u32,
    pub kind: InputDeviceKind,
    pub name: &'static str,
}

impl InputDevice {
    pub const fn new(id: u32, kind: InputDeviceKind, name: &'static str) -> Self {
        Self { id, kind, name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn device_kind_equality() {
        assert_eq!(InputDeviceKind::Joystick, InputDeviceKind::Joystick);
    }
}
