use std::time::Duration;

use rustgear::{Aircraft, FlightState, InputState, SimTime};

const FIXED_DT: f64 = 1.0 / 120.0;

fn main() {
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

    let mut state = FlightState {
        speed_kts: 80.0,
        altitude_ft: 1500.0,
        heading_deg: 0.0,
        pitch_deg: 6.0,
        roll_deg: 0.0,
        gamma_rad: 0.0,
        vertical_speed_fpm: 0.0,
        turn_rate_deg_per_s: 0.0,
        pitch_rate_deg_per_s: 0.0,
    };
    let input = InputState {
        throttle: 0.8,
        pitch: 0.1,
        roll: 0.0,
        yaw: 0.0,
    };
    let mut time = SimTime::new();

    println!("RustGear CLI log: aircraft={} model={}", plane.id, plane.name);
    println!(
        "t={:.2}s | speed={:.2}kt | alt={:.1}ft | hdg={:.1} | pitch={:.1}° | roll={:.1}°",
        time.sim_time, state.speed_kts, state.altitude_ft, state.heading_deg, state.pitch_deg, state.roll_deg
    );

    for step in 1..=1800 {
        plane.step(FIXED_DT, &input, &mut state);
        time.update(FIXED_DT);

        if step % 120 == 0 {
            println!(
                "t={:.2}s | speed={:.2}kt | alt={:.1}ft | hdg={:.1} | pitch={:.1}° | roll={:.1}°",
                time.sim_time, state.speed_kts, state.altitude_ft, state.heading_deg, state.pitch_deg, state.roll_deg
            );
        }

        std::thread::sleep(Duration::from_millis(8));
    }

    println!(
        "Final: t={:.2}s speed={:.2}kt alt={:.1}ft hdg={:.1}",
        time.sim_time, state.speed_kts, state.altitude_ft, state.heading_deg
    );
}
