use std::time::Instant;

#[derive(Debug, Clone)]
pub struct TimerManager {
    pub last: Instant,
    pub frame: u64,
    pub fps: f64,
    pub accumulated: f64,
}

impl TimerManager {
    pub fn new() -> Self {
        Self { last: Instant::now(), frame: 0, fps: 0.0, accumulated: 0.0 }
    }

    pub fn tick(&mut self, dt_real: f64) {
        self.frame = self.frame.wrapping_add(1);
        self.fps = 1.0 / dt_real.max(1e-9);
        self.last = Instant::now();
        self.accumulated += dt_real;
    }

    pub fn elapsed(&self) -> std::time::Duration {
        self.last.elapsed()
    }

    pub fn frame(&self) -> u64 {
        self.frame
    }
}

impl Default for TimerManager {
    fn default() -> Self {
        Self::new()
    }
}
