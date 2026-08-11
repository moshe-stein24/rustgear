use std::path::PathBuf;

use crate::{AircraftCatalog, AircraftConfig, AircraftMetadata};

pub fn load_catalog() -> AircraftCatalog {
    let mut catalog = AircraftCatalog::new();
    let dirs = vec![
        PathBuf::from("/usr/share/games/flightgear/Aircraft-aisim"),
        PathBuf::from("/usr/share/games/flightgear/Aircraft-uiuc"),
    ];
    for dir in dirs {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map(|e| e == "json").unwrap_or(false) {
                    if let Ok(meta) = AircraftMetadata::from_json5(&path) {
                        let model = meta.to_flight_model();
                        let cfg = AircraftConfig::new(&meta.id, &meta.id, model);
                        catalog.add(cfg);
                    }
                }
            }
        }
    }
    catalog
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn load_catalog_finds_c172p() {
        let catalog = load_catalog();
        assert!(catalog.find("c172p").is_some(), "c172p should be in catalog");
    }

    #[test]
    fn load_catalog_finds_f16() {
        let catalog = load_catalog();
        assert!(catalog.find("f16").is_some(), "f16 should be in catalog");
    }
}
