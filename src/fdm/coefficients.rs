use crate::aircraft::metadata::AircraftMetadata;

#[derive(Debug, Clone, Copy, Default)]
pub struct Coefficients {
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

impl Coefficients {
    pub fn from_metadata(meta: &AircraftMetadata) -> Self {
        Self {
            cl_min: meta.cl_min,
            cl_alpha: meta.cl_alpha,
            cl_q: meta.cl_q,
            cl_df: meta.cl_df,
            cd_min: meta.cd_min,
            cd_alpha: meta.cd_alpha,
            cd_beta: meta.cd_beta,
            cd_i: meta.cd_i,
            cd_df: meta.cd_df,
            cm_alpha: meta.cm_alpha,
            cm_q: meta.cm_q,
            cm_de: meta.cm_de,
            cy_beta: meta.cy_beta,
            cl_p: meta.cl_p,
            cn_beta: meta.cn_beta,
            cn_r: meta.cn_r,
        }
    }

    pub fn compute_cl(&self, alpha_rad: f64, q: f64, delta_flap: f64) -> f64 {
        self.cl_min + self.cl_alpha * alpha_rad + self.cl_q * q + self.cl_df * delta_flap
    }

    pub fn compute_cd(&self, cl: f64, beta: f64, delta_flap: f64) -> f64 {
        self.cd_min + self.cd_alpha * alpha_from_cl(cl, self.cl_alpha).abs() + self.cd_beta * beta.abs() + self.cd_i * cl.powi(2) + self.cd_df * delta_flap.powi(2)
    }
}

fn alpha_from_cl(cl: f64, cl_alpha: f64) -> f64 {
    if cl_alpha.abs() < 1e-9 { 0.0 } else { cl / cl_alpha }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn coefficients_from_metadata() {
        let meta = AircraftMetadata {
            id: "test".into(),
            sw: 16.2,
            cbar: 5.3,
            bw: 35.8,
            mass_kg: 1000.0,
            engine: vec![],
            gear: vec![],
            cl_min: 0.3,
            cl_alpha: 5.0,
            cl_q: 0.0,
            cl_df: 0.0,
            cd_min: 0.02,
            cd_alpha: 0.1,
            cd_beta: 0.0,
            cd_i: 0.04,
            cd_df: 0.0,
            cm_alpha: 0.0,
            cm_q: 0.0,
            cm_de: 0.0,
            cy_beta: 0.0,
            cl_p: 0.0,
            cn_beta: 0.0,
            cn_r: 0.0,
        };
        let coef = Coefficients::from_metadata(&meta);
        assert!((coef.cl_alpha - 5.0).abs() < 1e-9);
    }
}
