use rustgear::{Aircraft, FlightState, InputState};

#[test]
fn systems_consume_fuel_when_running() {
    let mut plane = Aircraft::new(
        "test",
        "Test",
        rustgear::FlightModel { mass_kg: 1000.0, wing_area_m2: 10.0, cd0: 0.04, k: 0.04, cl_alpha_per_rad: 5.0, thrust_n: 1000.0 },
    );
    let mut state = FlightState::default();
    let input = InputState { roll: 0.0, pitch: 0.0, yaw: 0.0, throttle: 1.0 };
    plane.step(10.0, &input, &mut state);
    assert!(plane.systems.fuel_remaining_kg < 100.0);
}

#[test]
fn systems_electrical_off_when_throttle_zero() {
    let mut plane = Aircraft::new(
        "test",
        "Test",
        rustgear::FlightModel { mass_kg: 1000.0, wing_area_m2: 10.0, cd0: 0.04, k: 0.04, cl_alpha_per_rad: 5.0, thrust_n: 0.0 },
    );
    let mut state = FlightState::default();
    let input = InputState { roll: 0.0, pitch: 0.0, yaw: 0.0, throttle: 0.0 };
    plane.step(1.0, &input, &mut state);
    assert!(!plane.systems.electrical_on);
}
