use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub fg_root: PathBuf,
    pub aircraft_dir: PathBuf,
    pub scenery_dir: Option<PathBuf>,
    pub tick_hz: u32,
    pub real_time: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            fg_root: PathBuf::from("/usr/share/games/flightgear"),
            aircraft_dir: PathBuf::from("/usr/share/games/flightgear/Aircraft"),
            scenery_dir: None,
            tick_hz: 60,
            real_time: true,
        }
    }
}

impl AppConfig {
    pub fn sim_dt(&self) -> f64 {
        if self.real_time { 1.0 / self.tick_hz as f64 } else { 0.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_sim_dt_matches_tick_hz() {
        let c = AppConfig::default();
        assert!((c.sim_dt() - 1.0/60.0).abs() < 1e-12);
    }
}
