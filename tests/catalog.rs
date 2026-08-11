use rustgear::{Aircraft, load_catalog};

#[test]
fn catalog_selects_c172p_and_flies() {
    let catalog = load_catalog();
    let entry = catalog.find("c172p").expect("c172p in catalog");
    let mut plane = Aircraft::new(&entry.id, &entry.name, entry.model);
    let mut state = rustgear::FlightState::default();
    let mut input = rustgear::InputState { throttle: 1.0, pitch: 0.1, ..Default::default() };
    for _ in 0..120 {
        plane.step(1.0/120.0, &input, &mut state);
    }
    assert!(state.speed_kts > 0.0, "catalog aircraft should fly");
}
