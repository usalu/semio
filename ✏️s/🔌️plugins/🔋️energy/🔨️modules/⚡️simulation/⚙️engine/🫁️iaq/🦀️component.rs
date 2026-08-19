//! 🫁️ Indoor air quality: CO₂ and generic contaminant mass balance with DCV.

use serde::{Deserialize, Serialize};

// #region 🔖️ContaminantState
/// 🫁️ Contaminant concentration state with history for transient integration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContaminantState {
    pub concentration_ppm: f64,
    pub history_ppm: [f64; 3],
}

impl ContaminantState {
    pub async fn new(concentration_ppm: f64) -> Self {
        Self { concentration_ppm, history_ppm: [concentration_ppm; 3] }
    }

    pub async fn push(&mut self, ppm: f64) {
        self.history_ppm = [ppm, self.history_ppm[0], self.history_ppm[1]];
        self.concentration_ppm = ppm;
    }
}
// #endregion 🔖️ContaminantState

// #region 🔖️ContaminantBalance
/// ⚖️ Generic contaminant mass balance inputs.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContaminantBalance {
    pub zone_volume_m3: f64,
    pub generation_rate_mg_s: f64,
    pub outdoor_concentration_ppm: f64,
    pub ventilation_flow_m3_s: f64,
    pub infiltration_flow_m3_s: f64,
    pub removal_rate_mg_s: f64,
    pub molecular_weight_g_mol: f64,
}

impl ContaminantBalance {
    pub async fn total_airflow_m3_s(&self) -> f64 {
        self.ventilation_flow_m3_s + self.infiltration_flow_m3_s
    }
}
// #endregion 🔖️ContaminantBalance

// #region 🔖️Co2Balance
/// 🫁️ CO₂-specific balance parameters per ASHRAE 62.1.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Co2Balance {
    pub zone_volume_m3: f64,
    pub occupancy: f64,
    pub co2_generation_per_person_mg_s: f64,
    pub outdoor_co2_ppm: f64,
    pub ventilation_flow_m3_s: f64,
    pub infiltration_flow_m3_s: f64,
}

impl Co2Balance {
    pub async fn generation_rate_mg_s(&self) -> f64 {
        self.occupancy * self.co2_generation_per_person_mg_s
    }

    pub async fn to_contaminant_balance(&self) -> ContaminantBalance {
        ContaminantBalance {
            zone_volume_m3: self.zone_volume_m3,
            generation_rate_mg_s: self.generation_rate_mg_s(),
            outdoor_concentration_ppm: self.outdoor_co2_ppm,
            ventilation_flow_m3_s: self.ventilation_flow_m3_s,
            infiltration_flow_m3_s: self.infiltration_flow_m3_s,
            removal_rate_mg_s: 0.0,
            molecular_weight_g_mol: 44.01,
        }
    }
}
// #endregion 🔖️Co2Balance

// #region 🔖️DcvControl
/// 🎛️ Demand-controlled ventilation setpoint.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct DcvControl {
    pub target_ppm: f64,
    pub min_flow_per_person_m3_s: f64,
    pub max_flow_per_person_m3_s: f64,
    pub outdoor_co2_ppm: f64,
}
// #endregion 🔖️DcvControl

// #region 🔖️Solvers
async fn ppm_to_mg_m3(ppm: f64, molecular_weight_g_mol: f64) -> f64 {
    ppm * molecular_weight_g_mol / 24.45
}

async fn mg_m3_to_ppm(mg_m3: f64, molecular_weight_g_mol: f64) -> f64 {
    mg_m3 * 24.45 / molecular_weight_g_mol
}

/// 📈️ Steady-state contaminant concentration [ppm].
pub async fn steady_state_concentration_ppm(balance: &ContaminantBalance) -> f64 {
    let q = balance.total_airflow_m3_s();
    if q < 1e-12 {
        return balance.outdoor_concentration_ppm;
    }
    let c_out = ppm_to_mg_m3(balance.outdoor_concentration_ppm, balance.molecular_weight_g_mol);
    let gen = balance.generation_rate_mg_s - balance.removal_rate_mg_s;
    let c_zone = c_out + gen / q;
    mg_m3_to_ppm(c_zone.max(0.0), balance.molecular_weight_g_mol)
}

/// ⏩️ Advance contaminant concentration one explicit Euler step [ppm].
pub async fn advance_contaminant(state: &ContaminantState, balance: &ContaminantBalance, dt_s: f64) -> f64 {
    if dt_s <= 0.0 || balance.zone_volume_m3 <= 0.0 {
        return state.concentration_ppm;
    }
    let c = ppm_to_mg_m3(state.concentration_ppm, balance.molecular_weight_g_mol);
    let c_out = ppm_to_mg_m3(balance.outdoor_concentration_ppm, balance.molecular_weight_g_mol);
    let q = balance.total_airflow_m3_s();
    let gen = balance.generation_rate_mg_s - balance.removal_rate_mg_s;
    let dc_dt = (q * (c_out - c) + gen) / balance.zone_volume_m3;
    let c_new = (c + dc_dt * dt_s).max(0.0);
    mg_m3_to_ppm(c_new, balance.molecular_weight_g_mol)
}

/// 🫁️ Steady-state CO₂ [ppm].
pub async fn steady_state_co2_ppm(balance: &Co2Balance) -> f64 {
    steady_state_concentration_ppm(&balance.to_contaminant_balance())
}

/// 🎛️ DCV required outdoor airflow per person [m³/s] from CO₂ mass balance.
pub async fn dcv_flow_per_person_m3_s(control: &DcvControl, occupancy: f64, indoor_co2_ppm: f64) -> f64 {
    if occupancy < 1e-6 {
        return control.min_flow_per_person_m3_s;
    }
    let delta_target = (control.target_ppm - control.outdoor_co2_ppm).max(50.0);
    let ratio = if indoor_co2_ppm > control.target_ppm { 1.0 + (indoor_co2_ppm - control.target_ppm) / delta_target } else { 1.0 };
    (control.min_flow_per_person_m3_s * ratio).clamp(control.min_flow_per_person_m3_s, control.max_flow_per_person_m3_s)
}

/// 🎛️ Required total DCV ventilation flow [m³/s].
pub async fn dcv_ventilation_flow_m3_s(control: &DcvControl, occupancy: f64, indoor_co2_ppm: f64) -> f64 {
    occupancy * dcv_flow_per_person_m3_s(control, occupancy, indoor_co2_ppm)
}
// #endregion 🔖️Solvers

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn co2_rises_with_occupancy_at_low_ventilation() {
        let balance = Co2Balance { zone_volume_m3: 200.0, occupancy: 10.0, co2_generation_per_person_mg_s: 7.0, outdoor_co2_ppm: 400.0, ventilation_flow_m3_s: 0.01, infiltration_flow_m3_s: 0.005 };
        let ppm = steady_state_co2_ppm(&balance);
        assert!(ppm > 400.0);
    }

    #[test]
    async fn contaminant_transient_approaches_steady_state() {
        let balance = ContaminantBalance { zone_volume_m3: 100.0, generation_rate_mg_s: 5.0, outdoor_concentration_ppm: 0.0, ventilation_flow_m3_s: 0.05, infiltration_flow_m3_s: 0.0, removal_rate_mg_s: 0.0, molecular_weight_g_mol: 44.01 };
        let ss = steady_state_concentration_ppm(&balance);
        let mut state = ContaminantState::new(0.0);
        for _ in 0..500 {
            let ppm = advance_contaminant(&state, &balance, 60.0);
            state.push(ppm);
        }
        assert!((state.concentration_ppm - ss).abs() / ss < 0.05);
    }

    #[test]
    async fn dcv_increases_flow_at_high_co2() {
        let ctrl = DcvControl { target_ppm: 1000.0, min_flow_per_person_m3_s: 0.00236, max_flow_per_person_m3_s: 0.01, outdoor_co2_ppm: 400.0 };
        let low = dcv_flow_per_person_m3_s(&ctrl, 5.0, 600.0);
        let high = dcv_flow_per_person_m3_s(&ctrl, 5.0, 1500.0);
        assert!(high > low);
    }
}
