use crate::scenery::Airport;

#[derive(Debug, Clone, Default)]
pub struct SceneryTile {
    pub airports: Vec<Airport>,
}

impl SceneryTile {
    pub fn load_navdata(&mut self, _base: &str) {
        // placeholder: parse FG navdata files into airports
    }

    pub fn find_nearest(&self, _lat: f64, _lon: f64) -> Option<&Airport> {
        self.airports.first()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tile_default_empty() {
        let tile = SceneryTile::default();
        assert!(tile.airports.is_empty());
    }

    #[test]
    fn nearest_placeholder() {
        let tile = SceneryTile::default();
        assert!(tile.find_nearest(0.0, 0.0).is_none());
    }
}
