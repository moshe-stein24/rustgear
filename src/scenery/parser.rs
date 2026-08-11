use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::scenery::Airport;

#[derive(Debug, Clone, Default)]
pub struct SceneryCatalog {
    pub airports: Vec<Airport>,
}

impl SceneryCatalog {
    pub fn load_apt_dat(&mut self, path: &Path) {
        if path.extension().map(|e| e == "gz").unwrap_or(false) {
            if let Ok(file) = File::open(path) {
                let gz = flate2::read::GzDecoder::new(file);
                let reader = BufReader::new(gz);
                self.parse_reader(reader);
            }
        } else if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);
            self.parse_reader(reader);
        }
    }

    fn parse_reader<R: BufRead>(&mut self, reader: R) {
        let mut current_airport: Option<Airport> = None;
        for line in reader.lines().flatten() {
            let trimmed = line.trim();
            if trimmed.starts_with("1 ") || trimmed.starts_with("1\t") {
                if let Some(ap) = current_airport.take() {
                    self.airports.push(ap);
                }
                current_airport = Self::parse_airport_header(trimmed);
            } else if trimmed.starts_with("100 ") || trimmed.starts_with("100\t") {
                if let Some((lat, lon, elev)) = Self::parse_runway_line(trimmed) {
                    if let Some(ref mut ap) = current_airport {
                        ap.lat_deg = lat;
                        ap.lon_deg = lon;
                        ap.elevation_ft = elev;
                    }
                }
            }
        }
        if let Some(ap) = current_airport {
            self.airports.push(ap);
        }
    }

    fn parse_airport_header(line: &str) -> Option<Airport> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 5 {
            return None;
        }
        let icao = parts[4];
        let name = parts.get(5).copied().unwrap_or("");
        Some(Airport::new(icao.to_string(), name.to_string(), 0.0, 0.0, 0.0))
    }

    fn parse_runway_line(line: &str) -> Option<(f64, f64, f64)> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 12 {
            return None;
        }
        let lat = parts[9].parse::<f64>().ok()?;
        let lon = parts[10].parse::<f64>().ok()?;
        let elev = parts[11].parse::<f64>().unwrap_or(0.0);
        Some((lat, lon, elev))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_airport_header() {
        let line = "1 15 1 0 VHXX [X] CLOSED Kai Tak";
        let ap = SceneryCatalog::parse_airport_header(line).unwrap();
        assert_eq!(ap.icao, "VHXX");
    }

    #[test]
    fn parse_runway_line() {
        let line = "100   54.86   2   0 0.00 1 3 0 13   22.32526300  114.19222700  563.88   60.05 3  0 0 2 31   22.30395000  114.21587400  220.98    0.00 3 1 0 2";
        let (lat, lon, elev) = SceneryCatalog::parse_runway_line(line).unwrap();
        assert!((lat - 22.32526300).abs() < 1e-6);
        assert!((lon - 114.19222700).abs() < 1e-6);
        assert!((elev - 563.88).abs() < 1e-6);
    }
}
