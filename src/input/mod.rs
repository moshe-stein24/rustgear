use crate::fdm::FlightState;

#[derive(Debug, Clone, Copy, Default)]
pub struct InputState {
    pub roll: f64,
    pub pitch: f64,
    pub yaw: f64,
    pub throttle: f64,
}

pub mod bindings;
pub mod device_manager;
pub mod devices;
pub mod event;

pub use bindings::{AxisBinding, InputAxis, InputBindingMap, InputButton};
pub use device_manager::InputDeviceManager;
pub use devices::{InputDevice, InputDeviceKind};
pub use event::{InputEvent, apply_events};
