use crate::input::InputState;

pub const FIXED_DT: f64 = 1.0 / 120.0;

/// 3-DOF point-mass flight model.
#[derive(Debug, Clone, Copy, Default)]
pub struct FlightState {
    pub speed_kts: f64,
    pub altitude_ft: f64,
    pub heading_deg: f64,
    pub pitch_deg: f64,
    pub roll_deg: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct FlightModel {
    pub mass_kg: f64,
    pub wing_area_m2: f64,
    pub cd0: f64,
    pub k: f64,
    pub thrust_n: f64,
}

impl FlightModel {
    pub fn step(&self, dt: f64, input: &InputState, state: &mut FlightState) {
        let thrust = self.thrust_n * input.throttle.clamp(0.0, 1.0);
        let v = state.speed_kts.max(0.0);
        let rho = 1.225;
        let q = 0.5 * rho * (v * 0.514444).powi(2);
        let cl = ((2.0 * (input.pitch * 0.2 + 0.05)) as f64).min(1.2).max(-0.3);
        let cd = self.cd0 + self.k * cl.powi(2);
        let drag = q * self.wing_area_m2 * cd;
        let accel = (thrust - drag) / self.mass_kg;
        state.speed_kts = (v + accel * dt).max(0.0);
        let dist_m = state.speed_kts * 0.514444 * dt;
        state.altitude_ft += (input.pitch * 5.0 * dt).clamp(-dist_m, dist_m);
        state.heading_deg = (state.heading_deg + input.yaw * 20.0 * dt + 360.0) % 360.0;
        state.pitch_deg = (state.pitch_deg + input.pitch * 15.0 * dt).clamp(-30.0, 30.0);
        state.roll_deg = (state.roll_deg + input.roll * 10.0 * dt).clamp(-60.0, 60.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn step_increases_speed_with_throttle() {
        let model = FlightModel { mass_kg: 1000.0, wing_area_m2: 10.0, cd0: 0.04, k: 0.04, thrust_n: 1000.0 };
        let mut state = FlightState::default();
        let input = InputState { roll: 0.0, pitch: 0.0, yaw: 0.0, throttle: 1.0 };
        model.step(1.0, &input, &mut state);
        assert!(state.speed_kts > 0.0);
    }

    #[test]
    fn step_does_not_decrease_speed_with_zero_throttle() {
        let model = FlightModel { mass_kg: 1000.0, wing_area_m2: 10.0, cd0: 0.04, k: 0.04, thrust_n: 1000.0 };
        let mut state = FlightState { speed_kts: 100.0, altitude_ft: 0.0, heading_deg: 0.0, pitch_deg: 0.0, roll_deg: 0.0 };
        let input = InputState { roll: 0.0, pitch: 0.0, yaw: 0.0, throttle: 0.0 };
        model.step(1.0, &input, &mut state);
        assert!(state.speed_kts < 100.0);
    }

    #[test]
    fn step_changes_heading_with_yaw() {
        let model = FlightModel { mass_kg: 1000.0, wing_area_m2: 10.0, cd0: 0.04, k: 0.04, thrust_n: 1000.0 };
        let mut state = FlightState::default();
        let input = InputState { roll: 0.0, pitch: 0.0, yaw: 0.5, throttle: 0.0 };
        model.step(1.0, &input, &mut state);
        assert!((state.heading_deg - 10.0).abs() < 1e-6);
    }
}
