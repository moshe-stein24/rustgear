use rustgear::{Aircraft, AppConfig, Condition, Environment, FlightState, InputState, InstrumentationState, Property, PropertyValue, SimTime};

#[test]
fn full_sim_step_changes_state() {
    let mut plane = Aircraft::new(
        "c172p",
        "Cessna 172P",
        rustgear::FlightModel {
            mass_kg: 1043.0,
            wing_area_m2: 16.2,
            cd0: 0.038,
            k: 0.04,
            thrust_n: 1100.0,
        },
    );
    let mut state = FlightState::default();
    let input = InputState { roll: 0.0, pitch: 0.1, yaw: 0.1, throttle: 1.0 };
    let mut instruments = InstrumentationState::default();
    let mut time = SimTime::new();

    plane.step(1.0, &input, &mut state);
    instruments.update(&state, 1.0);
    time.update(1.0);

    assert!(state.speed_kts > 0.0, "speed should increase with throttle");
    assert!(state.altitude_ft > 0.0, "altitude should increase with positive pitch");
    assert!((state.heading_deg - 2.0).abs() < 1e-6, "heading should change with yaw, got {}", state.heading_deg);
    assert!(plane.systems.fuel_remaining_kg < 100.0, "fuel should decrease");
    assert!(plane.systems.electrical_on, "electrical should be on with engine running");
    assert!(instruments.airspeed_kt > 0.0, "instruments should show airspeed");
    assert!(time.sim_time > 0.0, "sim time should advance");
}

#[test]
fn default_config_paths() {
    let cfg = AppConfig::default();
    assert!(cfg.fg_root.ends_with("flightgear"));
    assert!(cfg.aircraft_dir.ends_with("Aircraft"));
    assert_eq!(cfg.tick_hz, 60);
    assert!(cfg.real_time);
}

#[test]
fn environment_density_profile() {
    let env = Environment::default();
    let sea = env.density_kg_m3(0.0);
    let alt = env.density_kg_m3(5000.0);
    assert!(sea > alt);
}

#[test]
fn property_tree_condition_integration() {
    let mut root = Property::leaf("sim", PropertyValue::Float(0.0));
    root.add_child(Property::leaf("gear-down", PropertyValue::Float(1.0)));
    root.add_child(Property::leaf("speed-kt", PropertyValue::Float(120.0)));

    assert!(Condition::new("gear-down == 1.0").evaluate(&root));
    assert!(!Condition::new("speed-kt == 0.0").evaluate(&root));
}
