use std::path::PathBuf;

use crate::config::AppConfig;
use crate::fdm::FlightModel;
use crate::systems::SystemsState;

#[derive(Debug, Clone)]
pub struct AircraftConfig {
    pub id: String,
    pub name: String,
    pub model: FlightModel,
    pub systems: SystemsState,
    pub description: Option<String>,
    pub author: Option<String>,
}

impl AircraftConfig {
    pub fn new(id: &str, name: &str, model: FlightModel) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            model,
            systems: SystemsState::default(),
            description: None,
            author: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct AircraftCatalog {
    pub entries: Vec<AircraftConfig>,
}

impl AircraftCatalog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, cfg: AircraftConfig) {
        self.entries.push(cfg);
    }

    pub fn find(&self, id: &str) -> Option<&AircraftConfig> {
        self.entries.iter().find(|e| e.id == id)
    }

    pub fn load_from_dir(&mut self, _cfg: &AppConfig, _dir: &PathBuf) {
        // placeholder: scan aircraft directory and parse config files
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn catalog_find_works() {
        let mut cat = AircraftCatalog::new();
        let cfg = AircraftConfig::new("c172p", "Cessna 172P", FlightModel {
            mass_kg: 1043.0,
            wing_area_m2: 16.2,
            cbar: 1.5,
            cd0: 0.038,
            k: 0.04,
            cl_alpha_per_rad: 5.5,
            cm_alpha_per_rad: -0.6,
            cm_de_per_unit: 0.35,
            cm_q: -12.4,
            thrust_n: 1100.0,
        });
        cat.add(cfg);
        assert!(cat.find("c172p").is_some());
        assert!(cat.find("b777").is_none());
    }
}
