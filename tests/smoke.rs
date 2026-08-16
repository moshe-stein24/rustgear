use rustgear::{fdm::flight_model::FIXED_DT, Aircraft, AppConfig, Condition, Environment, FlightState, InputState, InstrumentationState, Property, PropertyValue, SimTime};

#[test]
fn full_sim_step_changes_state() {
    let mut plane = Aircraft::new(
        "c172p",
        "Cessna 172P",
        rustgear::FlightModel {
            mass_kg: 1043.0,
            wing_area_m2: 16.2,
            cbar: 1.5,
            cd0: 0.038,
            k: 0.04,
            cl_alpha_per_rad: 5.5,
            thrust_n: 1100.0,
            cm_alpha_per_rad: -0.6,
            cm_de_per_unit: 0.35,
            cm_q: -12.4,
        },
    );
    let mut state = FlightState { speed_kts: 80.0, altitude_ft: 1500.0, heading_deg: 0.0, pitch_deg: 6.0, roll_deg: 0.0, gamma_rad: 0.0, vertical_speed_fpm: 0.0, turn_rate_deg_per_s: 0.0, pitch_rate_deg_per_s: 0.0 };
    let input = InputState { roll: 0.0, pitch: 0.1, yaw: 0.1, throttle: 1.0 };
    let mut instruments = InstrumentationState::default();
    let mut time = SimTime::new();

    for _ in 0..120 {
        plane.step(FIXED_DT, &input, &mut state);
        instruments.update(&state, FIXED_DT);
        time.update(FIXED_DT);
    }

    assert!(state.speed_kts.is_finite() && state.speed_kts > 0.0, "simulation should advance speed, got {}", state.speed_kts);
    assert!(state.altitude_ft.is_finite(), "simulation should produce finite altitude, got {}", state.altitude_ft);
    assert!(state.heading_deg.is_finite() && state.heading_deg > 0.0, "simulation should advance heading, got {}", state.heading_deg);
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
