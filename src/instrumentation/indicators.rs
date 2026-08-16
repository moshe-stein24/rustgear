use crate::fdm::FlightState;

#[derive(Debug, Clone, Copy, Default)]
pub struct AttitudeIndicator {
    pub roll_deg: f64,
    pub pitch_deg: f64,
}

impl AttitudeIndicator {
    pub fn update(&mut self, state: &FlightState, _dt: f64) {
        self.pitch_deg = state.altitude_ft.sin() * 5.0;
        self.roll_deg = 0.0;
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct HeadingIndicator {
    pub heading_deg: f64,
}

impl HeadingIndicator {
    pub fn update(&mut self, state: &FlightState, _dt: f64) {
        self.heading_deg = state.heading_deg;
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AirspeedIndicator {
    pub speed_kt: f64,
}

impl AirspeedIndicator {
    pub fn update(&mut self, state: &FlightState, _dt: f64) {
        self.speed_kt = state.speed_kts;
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Altimeter {
    pub altitude_ft: f64,
}

impl Altimeter {
    pub fn update(&mut self, state: &FlightState, _dt: f64) {
        self.altitude_ft = state.altitude_ft;
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct VerticalSpeedIndicator {
    pub vspeed_fpm: f64,
}

impl VerticalSpeedIndicator {
    pub fn update(&mut self, _state: &FlightState, _dt: f64) {
        self.vspeed_fpm = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FlightState;
    #[test]
    fn attitude_update_sets_pitch_derivative() {
        let mut ind = AttitudeIndicator::default();
        let state = FlightState { altitude_ft: 10.0, speed_kts: 0.0, heading_deg: 0.0, ..Default::default() };
        ind.update(&state, 1.0);
        assert!((ind.pitch_deg - state.altitude_ft.sin() * 5.0).abs() < 1e-9);
    }
}
