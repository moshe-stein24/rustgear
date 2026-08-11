use crate::fdm::FlightModel;
use crate::{FlightState, InputState, systems::SystemsState};

#[derive(Debug, Clone)]
pub struct Aircraft {
    pub id: String,
    pub name: String,
    pub model: FlightModel,
    pub systems: SystemsState,
}

impl Aircraft {
    pub fn new(id: &str, name: &str, model: FlightModel) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            model,
            systems: SystemsState::default(),
        }
    }

    pub fn step(&mut self, dt: f64, input: &InputState, state: &mut FlightState) {
        self.model.step(dt, input, state);
        let engine_running = input.throttle > 0.05 && self.systems.fuel_remaining_kg > 0.0;
        self.systems.tick(dt, input.throttle, engine_running);
    }
}
