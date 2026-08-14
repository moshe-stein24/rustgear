use rustgear::{fdm::FlightModel, gui::RustGearApp};

fn main() -> eframe::Result {
    let plane = rustgear::Aircraft::new(
        "c172p",
        "Cessna 172P",
        FlightModel {
            mass_kg: 1043.0,
            wing_area_m2: 16.2,
            cd0: 0.038,
            k: 0.04,
            thrust_n: 1100.0,
        },
    );

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([420.0, 520.0])
            .with_title("RustGear - Instruments"),
        ..Default::default()
    };

    eframe::run_native("rustgear-instruments", options, Box::new(|_cc| {
        Ok(Box::new(RustGearApp::new(plane)))
    }))
}
