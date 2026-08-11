#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum System {
    Electrical,
    Hydraulic,
    Fuel,
    Avionics,
}

#[derive(Debug, Clone)]
pub struct SystemsState {
    pub electrical_on: bool,
    pub hydraulic_pressure_psi: f64,
    pub fuel_remaining_kg: f64,
    pub avionics_on: bool,
}

impl Default for SystemsState {
    fn default() -> Self {
        Self {
            electrical_on: false,
            hydraulic_pressure_psi: 0.0,
            fuel_remaining_kg: 100.0,
            avionics_on: false,
        }
    }
}

impl SystemsState {
    pub fn tick(&mut self, dt: f64, throttle: f64, engine_running: bool) {
        if engine_running {
            self.electrical_on = true;
            self.hydraulic_pressure_psi = (self.hydraulic_pressure_psi + (3000.0 - self.hydraulic_pressure_psi) * 0.5 * dt).min(3000.0);
            self.avionics_on = true;
            self.fuel_remaining_kg = (self.fuel_remaining_kg - throttle * 2.5 * dt).max(0.0);
        } else {
            self.electrical_on = false;
            self.hydraulic_pressure_psi = (self.hydraulic_pressure_psi * (-0.1 * dt).exp()).max(0.0);
            self.avionics_on = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn engine_running_sets_electrical_on() {
        let mut s = SystemsState::default();
        s.tick(1.0, 1.0, true);
        assert!(s.electrical_on);
    }

    #[test]
    fn engine_off_decays_hydraulics() {
        let mut s = SystemsState { hydraulic_pressure_psi: 3000.0, ..Default::default() };
        s.tick(1.0, 0.0, false);
        assert!(s.hydraulic_pressure_psi < 3000.0);
    }
}
