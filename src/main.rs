use rustgear::{aircraft::load_catalog, gui::RustGearApp};

fn main() -> eframe::Result {
    let debug = std::env::args().any(|a| a == "--debug");
    let catalog = load_catalog();
    let selected_index = 0;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([420.0, 520.0])
            .with_title("RustGear"),
        ..Default::default()
    };

    eframe::run_native("rustgear", options, Box::new(|_cc| {
        Ok(Box::new(RustGearApp::new(catalog, selected_index, debug)))
    }))
}
