use eframe::egui;

use crate::{FlightState, InstrumentationState, SystemsState};

#[derive(Debug, Default)]
pub struct RustGearGui {
    pub state: FlightState,
    pub instruments: InstrumentationState,
    pub systems: SystemsState,
}

impl RustGearGui {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    pub fn update_state(&mut self, state: FlightState, instruments: InstrumentationState, systems: SystemsState) {
        self.state = state;
        self.instruments = instruments;
        self.systems = systems;
    }
}

impl eframe::App for RustGearGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("View", |ui| {
                    if ui.button("Refresh").clicked() {
                        ui.close_menu();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("RustGear Instruments");
            ui.separator();

            egui::Grid::new("instruments").striped(true).show(ui, |ui| {
                ui.label("IAS");
                ui.label(format!("{:.1} kt", self.instruments.airspeed_kt));
                ui.end_row();

                ui.label("ALT");
                ui.label(format!("{:.1} ft", self.instruments.altitude_ft));
                ui.end_row();

                ui.label("HDG");
                ui.label(format!("{:.1} deg", self.instruments.heading_deg));
                ui.end_row();

                ui.label("SPD");
                ui.label(format!("{:.2} kt", self.state.speed_kts));
                ui.end_row();
            });

            ui.separator();
            ui.label(format!("Electrical: {}", self.systems.electrical_on));
            ui.label(format!("Hydraulic: {:.0} psi", self.systems.hydraulic_pressure_psi));
            ui.label(format!("Fuel: {:.2} kg", self.systems.fuel_remaining_kg));
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FlightState, InstrumentationState, SystemsState};
    #[test]
    fn gui_defaults() {
        let gui = RustGearGui::default();
        assert_eq!(gui.state.speed_kts, 0.0);
    }

    #[test]
    fn gui_update_state_changes_values() {
        let mut gui = RustGearGui::default();
        gui.update_state(
            FlightState { speed_kts: 120.0, altitude_ft: 5000.0, heading_deg: 90.0 },
            InstrumentationState { airspeed_kt: 120.0, altitude_ft: 5000.0, heading_deg: 90.0, turn_rate_deg_per_s: 0.0, vertical_speed_fpm: 0.0 },
            SystemsState { electrical_on: true, hydraulic_pressure_psi: 2000.0, fuel_remaining_kg: 80.0, avionics_on: true },
        );
        assert_eq!(gui.state.speed_kts, 120.0);
        assert!(gui.systems.electrical_on);
    }
}
