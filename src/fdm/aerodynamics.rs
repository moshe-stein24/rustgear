
#[derive(Debug, Clone, Copy, Default)]
pub struct AerodynamicState {
    pub cl: f64,
    pub cd: f64,
    pub cm: f64,
}

impl AerodynamicState {
    pub fn compute_cl(&self, alpha_rad: f64, c_l_alpha: f64, cl_min: f64) -> f64 {
        (cl_min + c_l_alpha * alpha_rad).clamp(-1.0, 2.0)
    }

    pub fn compute_cd(&self, cl: f64, cd0: f64, k: f64) -> f64 {
        cd0 + k * cl.powi(2)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Atmosphere {
    pub sea_level_pressure: f64,
    pub sea_level_temp: f64,
}

impl Default for Atmosphere {
    fn default() -> Self {
        Self { sea_level_pressure: 101325.0, sea_level_temp: 288.15 }
    }
}

impl Atmosphere {
    pub fn density(&self, altitude_ft: f64) -> f64 {
        let alt_m = altitude_ft * 0.3048;
        let temp = self.sea_level_temp - 0.0065 * alt_m;
        let pressure = self.sea_level_pressure * (temp / self.sea_level_temp).powf(5.2561);
        1.225 * (pressure / self.sea_level_pressure) * (self.sea_level_temp / temp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn density_decreases_with_altitude() {
        let atmo = Atmosphere::default();
        let sea = atmo.density(0.0);
        let alt = atmo.density(10000.0);
        assert!(alt < sea);
    }

    #[test]
    fn compute_cl_increases_with_alpha() {
        let state = AerodynamicState::default();
        let cl1 = state.compute_cl(0.0, 5.0, 0.3);
        let cl2 = state.compute_cl(0.1, 5.0, 0.3);
        assert!(cl2 > cl1);
    }

    #[test]
    fn compute_cd_quadratic() {
        let state = AerodynamicState::default();
        let cd1 = state.compute_cd(0.5, 0.02, 0.04);
        let cd2 = state.compute_cd(0.6, 0.02, 0.04);
        assert!(cd2 > cd1);
    }
}
