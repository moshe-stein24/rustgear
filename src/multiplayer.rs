use std::collections::HashMap;

use crate::FlightState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerId(pub u32);

#[derive(Debug, Clone, Default)]
pub struct MultiplayerState {
    pub players: HashMap<PlayerId, FlightState>,
}

impl MultiplayerState {
    pub fn update_player(&mut self, id: PlayerId, state: FlightState) {
        self.players.insert(id, state);
    }

    pub fn remove_player(&mut self, id: PlayerId) {
        self.players.remove(&id);
    }
}
