use std::time::{Duration, Instant};

use rustgear::{Aircraft, AppConfig, Environment, FlightState, InstrumentationState, InputState, SimTime};

const FIXED_DT: f64 = 1.0 / 120.0;

fn main() {
    let _cfg = AppConfig::default();
    let _env = Environment::default();
    let catalog = rustgear::load_catalog();
    let entry = catalog.find("c172p").or_else(|| catalog.find("f16")).expect("loaded catalog has aircraft");
    let mut plane = Aircraft::new(&entry.id, &entry.name, entry.model);
    let mut state = FlightState::default();
    let mut input = InputState::default();
    input.throttle = 1.0;
    input.pitch = 0.1;
    let mut instruments = InstrumentationState::default();
    let mut time = SimTime::new();
    let mut last = Instant::now();
    let mut accum = 0.0;

    println!("RustGear: aircraft={} model={}", plane.id, plane.name);
    for step in 1..=1800 {
        let now = Instant::now();
        let dt_real = now.duration_since(last).as_secs_f64();
        last = now;
        accum += dt_real;
        while accum >= FIXED_DT {
            accum -= FIXED_DT;
            plane.step(FIXED_DT, &input, &mut state);
            instruments.update(&state, FIXED_DT);
            time.update(FIXED_DT);
        }
        if step % 120 == 0 {
            println!(
                "t={:.2}s speed={:.2}kt alt={:.1}ft hdg={:.1} fuel={:.2}kg | IAS={:.2}kt ALT={:.1}ft",
                time.sim_time, state.speed_kts, state.altitude_ft, state.heading_deg, plane.systems.fuel_remaining_kg,
                instruments.airspeed_kt, instruments.altitude_ft
            );
        }
        std::thread::sleep(Duration::from_millis(16));
    }
}
