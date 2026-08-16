use rustgear::{FlightModel, FlightState, InputState};

#[test]
fn throttle_to_full_airspeed() {
    let model = FlightModel {
        mass_kg: 500.0,
        wing_area_m2: 10.0,
        cbar: 1.0,
        cd0: 0.04,
        k: 0.04,
        cl_alpha_per_rad: 5.0,
        thrust_n: 20000.0,
        cm_alpha_per_rad: -0.5,
        cm_de_per_unit: 0.35,
        cm_q: -12.4,
    };
    let mut state = FlightState { speed_kts: 60.0, ..FlightState::default() };
    let input = InputState { roll: 0.0, pitch: 0.0, yaw: 0.0, throttle: 1.0 };
    for _ in 0..600 {
        model.step(1.0/120.0, &input, &mut state);
    }
    assert!(state.speed_kts > 100.0, "expected speed > 100kt, got {}", state.speed_kts);
}

#[test]
fn zero_throttle_decelerates() {
    let model = FlightModel {
        mass_kg: 1000.0,
        wing_area_m2: 10.0,
        cbar: 1.0,
        cd0: 0.04,
        k: 0.04,
        cl_alpha_per_rad: 5.0,
        thrust_n: 0.0,
        cm_alpha_per_rad: -0.5,
        cm_de_per_unit: 0.35,
        cm_q: -12.4,
    };
    let mut state = FlightState { speed_kts: 120.0, altitude_ft: 1000.0, heading_deg: 0.0, pitch_deg: 2.0, roll_deg: 0.0, gamma_rad: 0.0, vertical_speed_fpm: 0.0, turn_rate_deg_per_s: 0.0, pitch_rate_deg_per_s: 0.0 };
    let input = InputState { roll: 0.0, pitch: 0.0, yaw: 0.0, throttle: 0.0 };
    model.step(10.0, &input, &mut state);
    assert!(state.speed_kts < 120.0, "expected deceleration, got {}", state.speed_kts);
}
