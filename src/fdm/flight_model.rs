use crate::input::InputState;

pub const FIXED_DT: f64 = 1.0 / 120.0;

#[derive(Debug, Clone, Copy, Default)]
pub struct FlightState {
    pub speed_kts: f64,
    pub altitude_ft: f64,
    pub heading_deg: f64,
    pub pitch_deg: f64,
    pub roll_deg: f64,
    pub gamma_rad: f64,
    pub vertical_speed_fpm: f64,
    pub turn_rate_deg_per_s: f64,
    pub pitch_rate_deg_per_s: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct FlightModel {
    pub mass_kg: f64,
    pub wing_area_m2: f64,
    pub cbar: f64,
    pub cd0: f64,
    pub k: f64,
    pub cl_alpha_per_rad: f64,
    pub thrust_n: f64,
    pub cm_alpha_per_rad: f64,
    pub cm_de_per_unit: f64,
    pub cm_q: f64,
}


impl FlightModel {
    pub fn step(&self, dt: f64, input: &InputState, state: &mut FlightState) {
        let throttle = input.throttle.clamp(0.0, 1.0);
        let v = state.speed_kts.max(0.0);
        let v_ms = v * 0.514444;
        let rho = 1.225;
        let q = 0.5 * rho * v_ms.powi(2);
        let pitch_rad = state.pitch_deg * std::f64::consts::PI / 180.0;
        let gamma = state.gamma_rad;
        let alpha = (pitch_rad - gamma).clamp(-0.4, 0.4);
        let cl = (self.cl_alpha_per_rad * alpha).clamp(-0.3, 1.2);
        let cd = self.cd0 + self.k * cl.powi(2);
        let lift = q * self.wing_area_m2 * cl;
        let drag = q * self.wing_area_m2 * cd;
        let thrust = self.thrust_n * throttle;
        let weight = self.mass_kg * 9.80665;
        let along = thrust * pitch_rad.cos() - drag - weight * gamma.sin();
        let normal = thrust * pitch_rad.sin() + lift - weight * gamma.cos();
        let dv = along / self.mass_kg;
        let dgamma = if v_ms > 1e-6 { (normal / (self.mass_kg * v_ms)).clamp(-1.2, 1.2) } else { 0.0 };
        let new_v_ms = (v_ms + dv * dt).max(0.0);
        let new_gamma = (gamma + dgamma * dt).clamp(-1.2, 1.2);
        let climb_rate = new_gamma.sin() * new_v_ms;
        let new_alt_m = (state.altitude_ft * 0.3048) + climb_rate * dt;
        let pitch_cmd = input.pitch * 20.0;
        let qbar = 0.5 * rho * v_ms.powi(2);
        let qhat = if self.cbar > 1e-6 { qbar * self.cbar / (2.0 * self.mass_kg) } else { 0.0 };
        let pitch_damping = self.cm_q * state.pitch_rate_deg_per_s;
        let pitch_static = self.cm_alpha_per_rad * alpha;
        let pitch_accel = pitch_cmd + pitch_static + pitch_damping;
        let new_pitch_rate = state.pitch_rate_deg_per_s + pitch_accel * qhat * dt;
        state.pitch_rate_deg_per_s = new_pitch_rate.clamp(-80.0, 80.0);
        state.pitch_deg = (state.pitch_deg + state.pitch_rate_deg_per_s * dt).clamp(-30.0, 30.0);
        let turn_accel = if v_ms > 1e-6 { (lift * (state.roll_deg * std::f64::consts::PI / 180.0).sin()) / self.mass_kg } else { 0.0 };
        let turn_rate_rad = if v_ms > 1e-6 { (turn_accel / v_ms).clamp(-1.5, 1.5) } else { 0.0 };
        state.speed_kts = new_v_ms / 0.514444;
        state.altitude_ft = new_alt_m / 0.3048;
        state.heading_deg = (state.heading_deg + turn_rate_rad * 180.0 / std::f64::consts::PI * dt + input.yaw * 20.0 * dt + 360.0) % 360.0;
        state.roll_deg = (state.roll_deg + input.roll * 12.0 * dt).clamp(-45.0, 45.0);
        state.gamma_rad = new_gamma;
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
            mass_kg: 1043.0,
            wing_area_m2: 16.2,
            cbar: 1.5,
            cd0: 0.038,
            k: 0.04,
            cl_alpha_per_rad: 5.143,
            thrust_n: 2738.32,
            cm_alpha_per_rad: -0.89,
            cm_de_per_unit: -1.28,
            cm_q: -12.4,
        };
        let mut state = FlightState { speed_kts: 80.0, altitude_ft: 1000.0, heading_deg: 0.0, pitch_deg: 6.0, roll_deg: 0.0, gamma_rad: 0.0, vertical_speed_fpm: 0.0, turn_rate_deg_per_s: 0.0, pitch_rate_deg_per_s: 0.0 };
        let input = InputState { roll: 0.0, pitch: 0.0, yaw: 0.0, throttle: 0.9 };
        for _ in 0..1200 {
            model.step(FIXED_DT, &input, &mut state);
        }
        assert!(state.speed_kts > 80.0, "speed should increase with throttle, got {}", state.speed_kts);
        assert!(state.altitude_ft >= 1000.0, "altitude should not drop below start with positive pitch, got {}", state.altitude_ft);
    }
}
