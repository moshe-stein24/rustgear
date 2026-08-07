// /home/moshe/flightgear_rust_rewrite/src/lib.rs

use std::fs;
use std::path::Path;

// Re-import necessary structs from lib.rs for compilation context
use crate::flightgear::{
    Configuration, RenderSettings, ScenerySettings, InputSettings, KeybindingMode, SystemStatus, Aircraft, ModelData, FlightState
};

// ==============================================================
// MODULE: Core Data Structures (The Immutable Foundation)
// Focus: Eliminating mutable state bugs from past iterations.
// ===================================================================

/// Defines fundamental flight parameters, ensuring immutability for simulation integrity.
#[derive(Debug, Clone)]
pub struct FlightState {
    pub speed_kts: f64,
    pub altitude_ft: f64,
    pub heading_deg: f64, // Degrees, 0-359
    pub fuel_remaining: f64,
    pub system_status: SystemStatus,
}

/// Enum to manage the status of FlightGear subsystems.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemStatus {
    EngineRunning,
    EngineOff,
    ATCActive,
    SceneryLoaded,
    SystemHalted,
}

/// Structure representing a single aircraft. This will be the core entity we manage.
#[derive(Debug, Clone)]
pub struct Aircraft {
    pub id: String, // e.g., "c172p"
    pub name: String,
    pub model_data: ModelData,
    pub current_state: FlightState,
}

/// Holds the specific aerodynamic and visual data for an aircraft.
#[derive(Debug, Clone)]
pub struct ModelData {
    pub mass_kg: f64,
    pub thrust_setting: f64, // Throttle input (0.0 to 1.0)
    pub max_speed_kts: f64,
    pub max_altitude_ft: f64,
}

/// Structure to hold all configuration parameters read from the .fgfsrc file.
#[derive(Debug, Clone)]
pub struct Configuration {
    pub fgfsrc_path: String,
    pub render_settings: RenderSettings, // Derived from --prop flags
    pub scenery_settings: ScenerySettings,
    pub input_settings: InputSettings,
}

/// Settings related to visual rendering and GPU optimization.
#[derive(Debug, Clone)]
pub struct RenderSettings {
    pub shadows_enabled: bool,
    pub shadow_detail: u8,
    pub clouds_3d: bool,
    pub particles_enabled: bool,
    pub particle_quality: u8,
    pub atmosphere_fog_enabled: bool,
}

/// Settings related to scenery loading and asset management.
#[derive(Debug, Clone)]
pub struct ScenerySettings {
    pub enable_clouds: bool,
    pub visibility_range: f64,
    pub render_quality: u8,
}

/// Settings related to user input mapping.
#[derive(Debug, Clone)]
pub struct InputSettings {
    pub motion_control_enabled: bool,
    pub keybinding_mode: KeybindingMode, // Enum for menu/shortcut handling
    pub use_remote_input: bool,
}

/// Enum to handle specific keybinding modes.
#[derive(Debug, Clone)]
pub enum KeybindingMode {
    Classic,     // Original Qt behavior
    Modern,      // Modernized mapping
    Custom,      // User-defined mappings
}

// Export core structs and traits
pub mod flightgear {
    pub use super::{Aircraft, FlightState, ModelData, Configuration, RenderSettings, ScenerySettings, InputSettings, KeybindingMode, SystemStatus};
}