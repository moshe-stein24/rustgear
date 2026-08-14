use crate::fdm::FlightState;

#[derive(Debug, Clone, Default)]
pub struct InstrumentationState {
    pub airspeed_kt: f64,
    pub altitude_ft: f64,
    pub heading_deg: f64,
    pub turn_rate_deg_per_s: f64,
    pub vertical_speed_fpm: f64,
    pub pitch_deg: f64,
    pub roll_deg: f64,
}

impl InstrumentationState {
    pub fn update(&mut self, state: &FlightState, dt: f64) {
        self.airspeed_kt = state.speed_kts;
        self.altitude_ft = state.altitude_ft;
        self.heading_deg = state.heading_deg;
        self.pitch_deg = state.pitch_deg;
        self.roll_deg = state.roll_deg;
        self.vertical_speed_fpm = ((state.altitude_ft - self.altitude_ft) / dt.max(1e-6)) * 60.0 * 0.3048;
        self.turn_rate_deg_per_s *= (-2.0 * dt).exp();
    }
}

pub mod indicators;

pub use indicators::{AirspeedIndicator, Altimeter, AttitudeIndicator, HeadingIndicator, VerticalSpeedIndicator};
