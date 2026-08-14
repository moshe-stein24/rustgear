use crate::input::InputState;

pub const FIXED_DT: f64 = 1.0 / 120.0;

/// 3-DOF wind-axis point-mass flight model with body-axis attitude/rates.
#[derive(Debug, Clone, Copy, Default)]
pub struct FlightState {
    pub speed_kts: f64,
    pub altitude_ft: f64,
    pub heading_deg: f64,
    pub pitch_deg: f64,
    pub roll_deg: f64,
    pub vertical_speed_fpm: f64,
    pub turn_rate_deg_per_s: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct FlightModel {
    pub mass_kg: f64,
    pub wing_area_m2: f64,
    pub cd0: f64,
    pub k: f64,
    pub cl_alpha_per_rad: f64,
    pub thrust_n: f64,
}

impl FlightModel {
    pub fn step(&self, dt: f64, input: &InputState, state: &mut FlightState) {
        let throttle = input.throttle.clamp(0.0, 1.0);
        let v = state.speed_kts.max(0.0);
        let v_ms = v * 0.514444;
        let rho = 1.225;
        let q = 0.5 * rho * v_ms.powi(2);
        let alpha_rad = (input.pitch * 0.25).clamp(-0.4, 0.4);
        let cl = (self.cl_alpha_per_rad * alpha_rad).clamp(-0.3, 1.8);
        let cd = self.cd0 + self.k * cl.powi(2);
        let lift = q * self.wing_area_m2 * cl;
        let drag = q * self.wing_area_m2 * cd;
        let thrust = self.thrust_n * throttle;
        let weight = self.mass_kg * 9.80665;
        let gamma_rad = (state.pitch_deg * std::f64::consts::PI / 180.0).sin()
            * (v_ms / (v_ms + 1e-6)).clamp(0.0, 1.0);
        let along = thrust - drag - weight * gamma_rad;
        let normal = lift - weight * (state.pitch_deg * std::f64::consts::PI / 180.0).cos();
        let accel = along / self.mass_kg;
        let new_v = (v_ms + accel * dt).max(0.0);
        let climb_rate = (normal / self.mass_kg) * v_ms;
        let new_alt_m = (state.altitude_ft * 0.3048) + climb_rate * dt;
        let turn_accel = if v_ms > 1e-6 {
            (lift * (state.roll_deg * std::f64::consts::PI / 180.0).sin()) / self.mass_kg
        } else {
            0.0
        };
        let turn_rate_rad = if v_ms > 1e-6 {
            (turn_accel / v_ms).clamp(-1.0, 1.0)
        } else {
            0.0
        };
        state.speed_kts = new_v / 0.514444;
        state.altitude_ft = new_alt_m / 0.3048;
        state.heading_deg = (state.heading_deg + turn_rate_rad * 180.0 / std::f64::consts::PI * dt + 360.0) % 360.0;
        state.pitch_deg = (state.pitch_deg + input.pitch * 12.0 * dt).clamp(-25.0, 25.0);
        state.roll_deg = (state.roll_deg + input.roll * 10.0 * dt).clamp(-45.0, 45.0);
        state.vertical_speed_fpm = climb_rate / 0.00508;
        state.turn_rate_deg_per_s = turn_rate_rad * 180.0 / std::f64::consts::PI;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn step_increases_speed_with_throttle() {
        let model = FlightModel {
            mass_kg: 1000.0,
            wing_area_m2: 10.0,
            cd0: 0.04,
            k: 0.04,
            cl_alpha_per_rad: 5.0,
            thrust_n: 1000.0,
        };
        let mut state = FlightState::default();
        let input = InputState {
            roll: 0.0,
            pitch: 0.0,
            yaw: 0.0,
            throttle: 1.0,
        };
        model.step(1.0, &input, &mut state);
        assert!(state.speed_kts > 0.0);
    }

    #[test]
    fn step_climbs_with_positive_pitch() {
        let model = FlightModel {
            mass_kg: 1000.0,
            wing_area_m2: 10.0,
            cd0: 0.04,
            k: 0.04,
            cl_alpha_per_rad: 5.0,
            thrust_n: 1000.0,
        };
        let mut state = FlightState::default();
        let input = InputState {
            roll: 0.0,
            pitch: 0.5,
            yaw: 0.0,
            throttle: 1.0,
        };
        for _ in 0..120 {
            model.step(FIXED_DT, &input, &mut state);
        }
        assert!(state.altitude_ft > 0.0, "altitude should increase with pitch, got {}", state.altitude_ft);
    }
}
