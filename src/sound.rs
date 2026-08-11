#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoundEvent {
    EngineStart,
    EngineStop,
    GearUp,
    GearDown,
    Crash,
}

#[derive(Debug, Clone, Default)]
pub struct SoundManager {
    pub enabled: bool,
    pub volume: f64,
}

impl SoundManager {
    pub fn new() -> Self {
        Self { enabled: true, volume: 1.0 }
    }

    pub fn play(&self, _event: SoundEvent) {
        if self.enabled {
            // placeholder: play sound sample
        }
    }

    pub fn set_volume(&mut self, volume: f64) {
        self.volume = volume.clamp(0.0, 1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn volume_clamped() {
        let mut mgr = SoundManager::new();
        mgr.set_volume(1.5);
        assert!((mgr.volume - 1.0).abs() < 1e-12);
    }

    #[test]
    fn disabled_does_not_play() {
        let mgr = SoundManager { enabled: false, ..Default::default() };
        mgr.play(SoundEvent::EngineStart);
    }
}
