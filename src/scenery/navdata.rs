use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct NavCatalog {
    pub navaids: Vec<NavAid>,
    pub fixes: Vec<Fix>,
    pub airways: Vec<Airway>,
}

#[derive(Debug, Clone)]
pub struct NavAid {
    pub ident: String,
    pub name: String,
    pub lat_deg: f64,
    pub lon_deg: f64,
    pub elev_ft: f64,
    pub freq: f64,
    pub range_nm: f64,
}

#[derive(Debug, Clone)]
pub struct Fix {
    pub ident: String,
    pub lat_deg: f64,
    pub lon_deg: f64,
}

#[derive(Debug, Clone)]
pub struct Airway {
    pub from_ident: String,
    pub from_lat: f64,
    pub from_lon: f64,
    pub to_ident: String,
    pub to_lat: f64,
    pub to_lon: f64,
    pub airway: String,
}

impl NavCatalog {
    pub fn load_navdat(&mut self, path: &Path) {
        if path.extension().map(|e| e == "gz").unwrap_or(false) {
            if let Ok(file) = File::open(path) {
                let reader = BufReader::new(flate2::read::GzDecoder::new(file));
                self.parse_nav(reader);
            }
        } else if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);
            self.parse_nav(reader);
        }
    }

    fn parse_nav<R: BufRead>(&mut self, reader: R) {
        for line in reader.lines().flatten() {
            let trimmed = line.trim();
            if trimmed.starts_with("2 ") || trimmed.starts_with("2\t") {
                self.navaids.push(Self::parse_navaid(trimmed));
            }
        }
    }

    fn parse_navaid(line: &str) -> NavAid {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let lat = parts.get(1).and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
        let lon = parts.get(2).and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
        let elev = parts.get(3).and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
        let freq = parts.get(4).and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
        let range_nm = parts.get(5).and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
        let ident = parts.get(7).copied().unwrap_or("").to_string();
        let name = parts.get(8).map(|s| *s).unwrap_or("").to_string();
        NavAid { ident, name, lat_deg: lat, lon_deg: lon, elev_ft: elev, freq, range_nm }
    }

    pub fn load_fixdat(&mut self, path: &Path) {
        if path.extension().map(|e| e == "gz").unwrap_or(false) {
            if let Ok(file) = File::open(path) {
                let reader = BufReader::new(flate2::read::GzDecoder::new(file));
                self.parse_fix(reader);
            }
        } else if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);
            self.parse_fix(reader);
        }
    }

    fn parse_fix<R: BufRead>(&mut self, reader: R) {
        for line in reader.lines().flatten() {
            let trimmed = line.trim();
            if trimmed.starts_with("I") || trimmed.starts_with("V") || trimmed.is_empty() {
                continue;
            }
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 3 {
                let lat = parts[0].parse::<f64>().unwrap_or(0.0);
                let lon = parts[1].parse::<f64>().unwrap_or(0.0);
                let ident = parts[2].to_string();
                self.fixes.push(Fix { ident, lat_deg: lat, lon_deg: lon });
            }
        }
    }

    pub fn load_awydat(&mut self, path: &Path) {
        if path.extension().map(|e| e == "gz").unwrap_or(false) {
            if let Ok(file) = File::open(path) {
                let reader = BufReader::new(flate2::read::GzDecoder::new(file));
                self.parse_awy(reader);
            }
        } else if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);
            self.parse_awy(reader);
        }
    }

    fn parse_awy<R: BufRead>(&mut self, reader: R) {
        for line in reader.lines().flatten() {
            let trimmed = line.trim();
            if trimmed.starts_with("I") || trimmed.starts_with("V") || trimmed.is_empty() {
                continue;
            }
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 9 {
                let from_lat = parts[1].parse::<f64>().unwrap_or(0.0);
                let from_lon = parts[2].parse::<f64>().unwrap_or(0.0);
                let to_ident = parts[3].to_string();
                let to_lat = parts[4].parse::<f64>().unwrap_or(0.0);
                let to_lon = parts[5].parse::<f64>().unwrap_or(0.0);
                let airway = parts[8].to_string();
                self.airways.push(Airway {
                    from_ident: "".to_string(),
                    from_lat,
                    from_lon,
                    to_ident,
                    to_lat,
                    to_lon,
                    airway,
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    #[test]
    fn load_navdat_finds_navaids() {
        let mut nav = NavCatalog::default();
        nav.load_navdat(&PathBuf::from("/usr/share/games/flightgear/Navaids/nav.dat.gz"));
        assert!(!nav.navaids.is_empty(), "nav.dat should load navaids");
        assert!(nav.navaids.iter().any(|n| n.ident == "APH"));
    }

    #[test]
    fn load_fixdat_finds_fixes() {
        let mut nav = NavCatalog::default();
        nav.load_fixdat(&PathBuf::from("/usr/share/games/flightgear/Navaids/fix.dat.gz"));
        assert!(!nav.fixes.is_empty(), "fix.dat should load fixes");
    }

    #[test]
    fn load_awydat_finds_airways() {
        let mut nav = NavCatalog::default();
        nav.load_awydat(&PathBuf::from("/usr/share/games/flightgear/Navaids/awy.dat.gz"));
        assert!(!nav.airways.is_empty(), "awy.dat should load airways");
    }
}
