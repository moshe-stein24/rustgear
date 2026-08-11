#[derive(Debug, Clone, PartialEq)]
pub struct Airport {
    pub icao: String,
    pub name: String,
    pub lat_deg: f64,
    pub lon_deg: f64,
    pub elevation_ft: f64,
}

impl Airport {
    pub fn new(icao: impl Into<String>, name: impl Into<String>, lat_deg: f64, lon_deg: f64, elevation_ft: f64) -> Self {
        Self {
            icao: icao.into(),
            name: name.into(),
            lat_deg,
            lon_deg,
            elevation_ft,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn airport_fields() {
        let a = Airport::new("EDDF", "Frankfurt", 50.0, 8.0, 364.0);
        assert_eq!(a.icao, "EDDF");
        assert_eq!(a.elevation_ft, 364.0);
    }
}
