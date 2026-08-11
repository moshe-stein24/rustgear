use crate::input::bindings::{InputAxis, InputBindingMap};

#[derive(Debug, Clone, Copy)]
pub struct InputEvent {
    pub axis: InputAxis,
    pub raw: f64,
}

pub fn apply_events(input: &mut crate::InputState, events: &[InputEvent], bindings: &InputBindingMap) {
    for event in events {
        let value = bindings.axis_value(event.axis, event.raw);
        match event.axis {
            InputAxis::Roll => input.roll = value,
            InputAxis::Pitch => input.pitch = value,
            InputAxis::Yaw => input.yaw = value,
            InputAxis::Throttle => input.throttle = value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::bindings::{AxisBinding, InputAxis};
    #[test]
    fn apply_events_updates_state() {
        let mut input = crate::InputState::default();
        let mut bindings = InputBindingMap::default();
        bindings.bind_axis(InputAxis::Throttle, AxisBinding::new(InputAxis::Throttle, 1.0, 0.0));
        let events = vec![InputEvent { axis: InputAxis::Throttle, raw: 0.8 }];
        apply_events(&mut input, &events, &bindings);
        assert!((input.throttle - 0.8).abs() < 1e-12);
    }
}
