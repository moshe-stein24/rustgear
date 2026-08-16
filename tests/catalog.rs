use rustgear::{load_catalog, FlightState, Aircraft, InputState};

#[test]
fn catalog_selects_c172p_and_f16() {
    let catalog = load_catalog();
    assert!(catalog.find("c172p").is_some(), "c172p in catalog");
    assert!(catalog.find("f16").is_some(), "f16 in catalog");
}

#[test]
fn catalog_selects_c172p_and_fly() {
    let catalog = load_catalog();
    let Some(entry) = catalog.find("c172p") else { panic!("c172p in catalog") };
    let mut plane = Aircraft::new(&entry.id, &entry.name, entry.model);
    let mut state = FlightState { speed_kts: 80.0, altitude_ft: 1500.0, heading_deg: 0.0, pitch_deg: 6.0, roll_deg: 0.0, gamma_rad: 0.0, vertical_speed_fpm: 0.0, turn_rate_deg_per_s: 0.0, pitch_rate_deg_per_s: 0.0 };
    let input = InputState { throttle: 1.0, pitch: 0.1, ..Default::default() };
    for _ in 0..1200 {
        plane.step(1.0/120.0, &input, &mut state);
    }
    assert!(state.speed_kts.is_finite() && state.speed_kts > 0.0, "catalog aircraft should produce finite speed, got {}", state.speed_kts);
    assert!(state.altitude_ft.is_finite(), "catalog aircraft should produce finite altitude, got {}", state.altitude_ft);
    assert!(state.heading_deg.is_finite() && (0.0..=360.0).contains(&state.heading_deg), "catalog aircraft should produce finite heading, got {}", state.heading_deg);
}
