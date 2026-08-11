use rustgear::gui::RustGearGui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([420.0, 520.0])
            .with_title("RustGear - Instruments"),
        ..Default::default()
    };
    eframe::run_native("rustgear-instruments", options, Box::new(|_cc| Ok(Box::new(RustGearGui::new(_cc)))))
}
