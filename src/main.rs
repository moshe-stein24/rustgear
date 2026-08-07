use eframe::egui;

struct RustGearLauncher {
    selected_aircraft: String,
    aircraft_list: Vec<String>,
    settings_open: bool,
}

impl RustGearLauncher {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            selected_aircraft: "c172p".to_string(),
            aircraft_list: vec![
                "c172p".to_string(),
                "f16-block-52".to_string(),
                "777-200".to_string(),
            ],
            settings_open: false,
        }
    }
}

impl eframe::App for RustGearLauncher {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("Settings", |ui| {
                    if ui.button("Configuration").clicked() {
                        self.settings_open = !self.settings_open;
                        ui.close_menu();
                    }
                });
                ui.menu_button("Help", |ui| {
                    ui.label("RustGear Launcher v0.1.0");
                    ui.label("FlightGear rewrite in Rust");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("RustGear - FlightGear Rewrite");
            ui.separator();

            ui.label("Select Aircraft:");
            egui::ComboBox::from_label("Aircraft")
                .selected_text(&self.selected_aircraft)
                .show_ui(ui, |ui| {
                    for aircraft in &self.aircraft_list {
                        ui.selectable_value(&mut self.selected_aircraft, aircraft.clone(), aircraft);
                    }
                });

            ui.separator();
            ui.label(format!("Selected: {}", self.selected_aircraft));

            if ui.button("Start FlightGear").clicked() {
                println!("Starting with aircraft: {}", self.selected_aircraft);
            }

            if self.settings_open {
                egui::Window::new("Settings")
                    .open(&mut self.settings_open)
                    .show(ctx, |ui| {
                        ui.label("Configuration options will go here.");
                        ui.label("Graphics, Input, Scenery settings.");
                    });
            }
        });

        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.label("Ready");
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 500.0])
            .with_title("RustGear Launcher"),
        ..Default::default()
    };

    eframe::run_native(
        "rustgear-launcher",
        options,
        Box::new(|cc| Ok(Box::new(RustGearLauncher::new(cc)))),
    )
}
