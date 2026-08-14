use std::fs;
use std::path::Path;

use crate::fdm::FlightModel;

#[derive(Debug, Clone)]
pub struct Engine {
    pub pos: [f64; 3],
    pub dir: [f64; 3],
    pub ft_max: f64,
    pub mt_max: f64,
    pub rpm_max: f64,
}

#[derive(Debug, Clone)]
pub struct Gear {
    pub pos: [f64; 3],
    pub spring: f64,
    pub damp: f64,
}

#[derive(Debug, Clone)]
pub struct AircraftMetadata {
    pub id: String,
    pub sw: f64,
    pub cbar: f64,
    pub bw: f64,
    pub mass_kg: f64,
    pub engine: Vec<Engine>,
    pub gear: Vec<Gear>,
    pub cl_min: f64,
    pub cl_alpha: f64,
    pub cl_q: f64,
    pub cl_df: f64,
    pub cd_min: f64,
    pub cd_alpha: f64,
    pub cd_beta: f64,
    pub cd_i: f64,
    pub cd_df: f64,
    pub cm_alpha: f64,
    pub cm_q: f64,
    pub cm_de: f64,
    pub cy_beta: f64,
    pub cl_p: f64,
    pub cn_beta: f64,
    pub cn_r: f64,
}

fn as_f64_array(arr: &serde_json::Value) -> [f64; 3] {
    match arr.as_array() {
        Some(a) if a.len() == 3 => [a[0].as_f64().unwrap_or(0.0), a[1].as_f64().unwrap_or(0.0), a[2].as_f64().unwrap_or(0.0)],
        _ => [0.0; 3],
    }
}

impl AircraftMetadata {
    pub fn from_json5(path: &Path) -> Result<Self, String> {
        let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let v: serde_json::Value = json5::from_str(&text).map_err(|e| e.to_string())?;
        let id = v.get("id").and_then(|x| x.as_str()).map(|s| s.to_string())
            .unwrap_or_else(|| path.file_stem().and_then(|s| s.to_str()).unwrap_or("unknown").to_string());
        let sw = v.get("Sw").and_then(|x| x.as_f64()).unwrap_or(0.0);
        let cbar = v.get("cbar").and_then(|x| x.as_f64()).unwrap_or(0.0);
        let bw = v.get("bw").and_then(|x| x.as_f64()).unwrap_or(0.0);
        let mass_kg = v.get("mass").and_then(|x| x.as_f64()).unwrap_or(0.0);
        let engines = v.get("engine").and_then(|x| x.as_array()).map(|arr| arr.iter().filter_map(|e| {
            let pos = as_f64_array(e.get("pos")?);
            let dir = as_f64_array(e.get("dir")?);
            let ft_max = e.get("FT_max").and_then(|x| x.as_f64()).unwrap_or(0.0);
            let mt_max = e.get("MT_max").and_then(|x| x.as_f64()).unwrap_or(0.0);
            let rpm_max = e.get("rpm_max").and_then(|x| x.as_f64()).unwrap_or(0.0);
            Some(Engine { pos, dir, ft_max, mt_max, rpm_max })
        }).collect()).unwrap_or_default();
        let gears = v.get("gear").and_then(|x| x.as_array()).map(|arr| arr.iter().filter_map(|g| {
            let pos = as_f64_array(g.get("pos")?);
            let spring = g.get("spring").and_then(|x| x.as_f64()).unwrap_or(0.0);
            let damp = g.get("damp").and_then(|x| x.as_f64()).unwrap_or(0.0);
            Some(Gear { pos, spring, damp })
        }).collect()).unwrap_or_default();
        let cl_min = v.get("CLmin").and_then(|x| x.as_f64()).unwrap_or(0.0);
        let cl_alpha = v.get("CLa").and_then(|x| x.as_f64()).unwrap_or(0.0);
        let cl_q = v.get("CLq").and_then(|x| x.as_f64()).unwrap_or(0.0);
        let cl_df = v.get("CLdf").and_then(|x| x.as_f64()).unwrap_or(0.0);
        let cd_min = v.get("CDmin").and_then(|x| x.as_f64()).unwrap_or(0.0);
        let cd_alpha = v.get("CDa").and_then(|x| x.as_f64()).unwrap_or(0.0);
        let cd_beta = v.get("CDb").and_then(|x| x.as_f64()).unwrap_or(0.0);
        let cd_i = v.get("CDi").and_then(|x| x.as_f64()).unwrap_or(0.0);
        let cd_df = v.get("CDdf").and_then(|x| x.as_f64()).unwrap_or(0.0);
        let cm_alpha = v.get("Cma").and_then(|x| x.as_f64()).unwrap_or(0.0);
        let cm_q = v.get("Cmq").and_then(|x| x.as_f64()).unwrap_or(0.0);
        let cm_de = v.get("Cmde").and_then(|x| x.as_f64()).unwrap_or(0.0);
        let cy_beta = v.get("CYb").and_then(|x| x.as_f64()).unwrap_or(0.0);
        let cl_p = v.get("Clp").and_then(|x| x.as_f64()).unwrap_or(0.0);
        let cn_beta = v.get("Cnb").and_then(|x| x.as_f64()).unwrap_or(0.0);
        let cn_r = v.get("Cnr").and_then(|x| x.as_f64()).unwrap_or(0.0);
        Ok(Self { id, sw, cbar, bw, mass_kg, engine: engines, gear: gears, cl_min, cl_alpha, cl_q, cl_df, cd_min, cd_alpha, cd_beta, cd_i, cd_df, cm_alpha, cm_q, cm_de, cy_beta, cl_p, cn_beta, cn_r })
    }

    pub fn to_flight_model(&self) -> FlightModel {
        let thrust_n = self.engine.first().map(|e| e.ft_max).unwrap_or(0.0);
        FlightModel {
            mass_kg: self.mass_kg,
            wing_area_m2: self.sw,
            cd0: self.cd_min,
            k: 0.04,
            cl_alpha_per_rad: self.cl_alpha,
            thrust_n,
        }
    }
}
