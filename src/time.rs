use std::time::Instant;

#[derive(Debug, Clone, Copy)]
pub struct SimTime {
    pub real_start: Instant,
    pub sim_time: f64,
    pub paused: bool,
    pub speedup: f64,
}

impl SimTime {
    pub fn new() -> Self {
        Self { real_start: Instant::now(), sim_time: 0.0, paused: false, speedup: 1.0 }
    }

    pub fn update(&mut self, dt_real: f64) {
        if !self.paused {
            self.sim_time += dt_real * self.speedup;
        }
    }

    pub fn elapsed_real(&self) -> f64 {
        self.real_start.elapsed().as_secs_f64()
    }

    pub fn sim_dt(&self, dt_real: f64) -> f64 {
        if self.paused { 0.0 } else { dt_real * self.speedup }
    }

    pub fn set_speedup(&mut self, speedup: f64) {
        self.speedup = speedup.max(0.0);
    }
}

impl Default for SimTime {
    fn default() -> Self {
        Self::new()
    }
}
