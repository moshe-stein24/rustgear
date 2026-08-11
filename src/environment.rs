#[derive(Debug, Clone, Copy)]
pub struct Environment {
    pub sea_level_pressure_pa: f64,
    pub sea_level_temp_k: f64,
    pub wind_from_deg: f64,
    pub wind_speed_kt: f64,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            sea_level_pressure_pa: 101325.0,
            sea_level_temp_k: 288.15,
            wind_from_deg: 0.0,
            wind_speed_kt: 0.0,
        }
    }
}

impl Environment {
    pub fn density_kg_m3(&self, altitude_ft: f64) -> f64 {
        let alt_m = altitude_ft * 0.3048;
        let scale = 0.0065;
        let temp = self.sea_level_temp_k - scale * alt_m;
        let pressure = self.sea_level_pressure_pa * (temp / self.sea_level_temp_k).powf(5.2561);
        1.225 * (pressure / self.sea_level_pressure_pa) * (self.sea_level_temp_k / temp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn density_decreases_with_altitude() {
        let env = Environment::default();
        let sea = env.density_kg_m3(0.0);
        let alt = env.density_kg_m3(10000.0);
        assert!(alt < sea);
    }
}
