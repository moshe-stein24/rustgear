use crate::{Condition, InputState, Property};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutopilotMode {
    Off,
    Heading,
    Altitude,
    Speed,
}

#[derive(Debug, Clone)]
pub struct AutopilotState {
    pub mode: AutopilotMode,
    pub target_heading_deg: f64,
    pub target_altitude_ft: f64,
    pub target_speed_kt: f64,
}

impl Default for AutopilotState {
    fn default() -> Self {
        Self {
            mode: AutopilotMode::Off,
            target_heading_deg: 0.0,
            target_altitude_ft: 0.0,
            target_speed_kt: 0.0,
        }
    }
}

impl AutopilotState {
    pub fn update(&mut self, _dt: f64, state: &crate::fdm::FlightState, input: &mut InputState) {
        match self.mode {
            AutopilotMode::Off => {}
            AutopilotMode::Heading => {
                let mut diff = self.target_heading_deg - state.heading_deg;
                if diff > 180.0 { diff -= 360.0; }
                if diff < -180.0 { diff += 360.0; }
                input.yaw = diff.clamp(-1.0, 1.0) * 0.5;
            }
            AutopilotMode::Altitude => {
                let diff = self.target_altitude_ft - state.altitude_ft;
                input.pitch = diff.clamp(-1.0, 1.0) * 0.05;
            }
            AutopilotMode::Speed => {
                let diff = self.target_speed_kt - state.speed_kts;
                input.throttle = (input.throttle + diff * 0.01).clamp(0.0, 1.0);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Autopilot {
    pub state: AutopilotState,
    pub conditions: Vec<Condition<'static>>,
}

impl Autopilot {
    pub fn new() -> Self {
        Self { state: AutopilotState::default(), conditions: Vec::new() }
    }

    pub fn add_condition(&mut self, cond: Condition<'static>) {
        self.conditions.push(cond);
    }

    pub fn evaluate(&self, root: &Property) -> bool {
        self.conditions.iter().any(|c| c.evaluate(root))
    }
}

impl Default for Autopilot {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FlightState, InputState};
    #[test]
    fn heading_mode_applies_yaw() {
        let mut ap = Autopilot::new();
        ap.state.mode = AutopilotMode::Heading;
        ap.state.target_heading_deg = 90.0;
        let state = FlightState { heading_deg: 0.0, speed_kts: 100.0, altitude_ft: 5000.0 };
        let mut input = InputState::default();
        ap.state.update(1.0, &state, &mut input);
        assert!(input.yaw > 0.0, "expected positive yaw to turn towards 90deg, got {}", input.yaw);
    }

    #[test]
    fn altitude_mode_applies_pitch() {
        let mut ap = Autopilot::new();
        ap.state.mode = AutopilotMode::Altitude;
        ap.state.target_altitude_ft = 6000.0;
        let state = FlightState { heading_deg: 0.0, speed_kts: 100.0, altitude_ft: 5000.0 };
        let mut input = InputState::default();
        ap.state.update(1.0, &state, &mut input);
        assert!(input.pitch > 0.0, "expected positive pitch to climb, got {}", input.pitch);
    }

    #[test]
    fn speed_mode_applies_throttle() {
        let mut ap = Autopilot::new();
        ap.state.mode = AutopilotMode::Speed;
        ap.state.target_speed_kt = 200.0;
        let state = FlightState { heading_deg: 0.0, speed_kts: 100.0, altitude_ft: 5000.0 };
        let mut input = InputState { throttle: 0.0, ..Default::default() };
        ap.state.update(1.0, &state, &mut input);
        assert!(input.throttle > 0.0, "expected throttle increase, got {}", input.throttle);
    }
}
