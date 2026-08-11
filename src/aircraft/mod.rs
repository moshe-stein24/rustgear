pub mod aircraft;
pub mod config;
pub mod loader;
pub mod metadata;

pub use aircraft::Aircraft;
pub use config::{AircraftCatalog, AircraftConfig};
pub use loader::load_catalog;
pub use metadata::AircraftMetadata;
