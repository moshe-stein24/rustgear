use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputAxis {
    Roll,
    Pitch,
    Yaw,
    Throttle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputButton {
    GearUp,
    GearDown,
    Brake,
    TrimUp,
    TrimDown,
}

#[derive(Debug, Clone)]
pub struct AxisBinding {
    pub axis: InputAxis,
    pub scale: f64,
    pub deadzone: f64,
}

impl AxisBinding {
    pub fn new(axis: InputAxis, scale: f64, deadzone: f64) -> Self {
        Self { axis, scale, deadzone }
    }

    pub fn apply(&self, raw: f64) -> f64 {
        let clamped = raw.clamp(-1.0, 1.0);
        if clamped.abs() < self.deadzone {
            return 0.0;
        }
        (clamped - clamped.signum() * self.deadzone) / (1.0 - self.deadzone) * self.scale
    }
}

#[derive(Debug, Clone, Default)]
pub struct InputBindingMap {
    pub axes: HashMap<InputAxis, AxisBinding>,
    pub buttons: HashMap<InputButton, bool>,
}

impl InputBindingMap {
    pub fn bind_axis(&mut self, axis: InputAxis, binding: AxisBinding) {
        self.axes.insert(axis, binding);
    }

    pub fn axis_value(&self, axis: InputAxis, raw: f64) -> f64 {
        self.axes.get(&axis).map(|b| b.apply(raw)).unwrap_or(0.0)
    }

    pub fn button_pressed(&self, button: InputButton) -> bool {
        self.buttons.get(&button).copied().unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn axis_deadzone_zeros_small_input() {
        let b = AxisBinding::new(InputAxis::Throttle, 1.0, 0.1);
        assert!((b.apply(0.05) - 0.0).abs() < 1e-12);
    }

    #[test]
    fn axis_scaling_works() {
        let b = AxisBinding::new(InputAxis::Roll, 2.0, 0.0);
        assert!((b.apply(0.5) - 1.0).abs() < 1e-12);
    }
}
