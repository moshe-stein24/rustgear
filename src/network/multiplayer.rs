use std::collections::HashMap;

use crate::{FlightState, PlayerId};

#[derive(Debug, Clone, Default)]
pub struct MultiplayerSync {
    pub states: HashMap<PlayerId, FlightState>,
}

impl MultiplayerSync {
    pub fn update(&mut self, id: PlayerId, state: FlightState) {
        self.states.insert(id, state);
    }

    pub fn remove(&mut self, id: PlayerId) {
        self.states.remove(&id);
    }

    pub fn snapshot(&self) -> &HashMap<PlayerId, FlightState> {
        &self.states
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FlightState, PlayerId};
    #[test]
    fn sync_update_snapshot() {
        let mut sync = MultiplayerSync::default();
        let state = FlightState { speed_kts: 100.0, altitude_ft: 5000.0, heading_deg: 90.0, ..Default::default() };
        sync.update(PlayerId(1), state);
        assert!(sync.snapshot().contains_key(&PlayerId(1)));
    }
}
