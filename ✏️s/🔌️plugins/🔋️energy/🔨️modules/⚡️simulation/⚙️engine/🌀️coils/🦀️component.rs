//! 🔥️❄️ Heating and cooling coils: electric, gas, water, DX with bypass factor.

use crate::curves::PerformanceCurve;
use crate::props::{latent_heat_vaporization, moist_air_enthalpy_j_per_kg, saturation_pressure_pa};
use crate::units::{CP_DRY_AIR, H_FG_0C};
use serde::{Deserialize, Serialize};

// #region 🔖️HeatingCoil
/// 🔥️ Heating coil types and ratings.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum HeatingCoil {
    Electric { capacity_w: f64, efficiency: f64 },
    Gas { capacity_w: f64, efficiency: f64 },
    HotWater { ua_w_per_k: f64, water_inlet_c: f64, water_flow_kg_s: f64, water_cp: f64 },
    Steam { capacity_w: f64, latent_fraction: f64 },
}

/// 📥️ Heating coil inlet air state.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CoilAirState {
    pub temperature_c: f64,
    pub humidity_ratio: f64,
    pub mass_flow_kg_s: f64,
    pub pressure_pa: f64,
}

/// 📤️ Heating coil output.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct HeatingCoilOutput {
    pub outlet: CoilAirState,
    pub total_heating_w: f64,
    pub gas_consumption_w: f64,
    pub water_heat_removal_w: f64,
}
// #endregion 🔖️HeatingCoil

// #region 🔖️CoolingCoil
/// ❄️ Cooling coil types including DX stages.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CoolingCoil {
    ChilledWater { ua_w_per_k: f64, water_inlet_c: f64, water_flow_kg_s: f64, water_cp: f64 },
    DxSingleSpeed { rated_capacity_w: f64, rated_shr: f64, cop_curve: PerformanceCurve },
    DxMultiSpeed { stages: Vec<DxStage> },
    DxVariableSpeed { rated_capacity_w: f64, rated_cop: f64, cop_curve: PerformanceCurve },
}

/// ❄️ DX compressor stage.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DxStage {
    pub capacity_w: f64,
    pub cop: f64,
    pub shr: f64,
}

/// 📤️ Cooling coil output.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CoolingCoilOutput {
    pub outlet: CoilAirState,
    pub total_cooling_w: f64,
    pub sensible_cooling_w: f64,
    pub latent_cooling_w: f64,
    pub compressor_power_w: f64,
    pub condensate_kg_s: f64,
}
// #endregion 🔖️CoolingCoil

// #region 🔖️HeatingOutput
/// 🔥️ Compute heating coil delivered capacity and outlet air state [W].
pub fn heating_coil_output_w(coil: &HeatingCoil, inlet: &CoilAirState, load_w: f64) -> HeatingCoilOutput {
    let m_dot = inlet.mass_flow_kg_s.max(0.0);
    if m_dot < 1e-9 || load_w <= 0.0 {
        return HeatingCoilOutput { outlet: *inlet, total_heating_w: 0.0, gas_consumption_w: 0.0, water_heat_removal_w: 0.0 };
    }

    match coil {
        HeatingCoil::Electric { capacity_w, efficiency } => {
            let q = load_w.min(*capacity_w);
            let delta_t = q / (m_dot * CP_DRY_AIR);
            HeatingCoilOutput { outlet: CoilAirState { temperature_c: inlet.temperature_c + delta_t, ..*inlet }, total_heating_w: q, gas_consumption_w: q / efficiency.max(0.01), water_heat_removal_w: 0.0 }
        }
        HeatingCoil::Gas { capacity_w, efficiency } => {
            let q = load_w.min(*capacity_w);
            let delta_t = q / (m_dot * CP_DRY_AIR);
            HeatingCoilOutput { outlet: CoilAirState { temperature_c: inlet.temperature_c + delta_t, ..*inlet }, total_heating_w: q, gas_consumption_w: q / efficiency.max(0.01), water_heat_removal_w: 0.0 }
        }
        HeatingCoil::HotWater { ua_w_per_k, water_inlet_c, water_flow_kg_s: _, water_cp: _ } => {
            let q_max = ua_w_per_k * (water_inlet_c - inlet.temperature_c).max(0.0);
            let q = load_w.min(q_max);
            let delta_t = q / (m_dot * CP_DRY_AIR);
            let water_q = q;
            HeatingCoilOutput { outlet: CoilAirState { temperature_c: inlet.temperature_c + delta_t, ..*inlet }, total_heating_w: q, gas_consumption_w: 0.0, water_heat_removal_w: water_q }
        }
        HeatingCoil::Steam { capacity_w, latent_fraction } => {
            let q = load_w.min(*capacity_w);
            let delta_t = q / (m_dot * CP_DRY_AIR);
            let humid_add = *latent_fraction * q / H_FG_0C * m_dot / m_dot.max(1.0);
            HeatingCoilOutput { outlet: CoilAirState { temperature_c: inlet.temperature_c + delta_t, humidity_ratio: inlet.humidity_ratio + humid_add * 0.001, ..*inlet }, total_heating_w: q, gas_consumption_w: 0.0, water_heat_removal_w: 0.0 }
        }
    }
}
// #endregion 🔖️HeatingOutput

// #region 🔖️CoolingOutput
/// ❄️ Compute cooling coil capacity with bypass factor and wet/dry behavior [W].
pub fn cooling_coil_output_w(coil: &CoolingCoil, inlet: &CoilAirState, load_w: f64, bypass_factor: f64) -> CoolingCoilOutput {
    let m_dot = inlet.mass_flow_kg_s.max(0.0);
    let bf = bypass_factor.clamp(0.0, 0.95);
    if m_dot < 1e-9 || load_w <= 0.0 {
        return CoolingCoilOutput { outlet: *inlet, total_cooling_w: 0.0, sensible_cooling_w: 0.0, latent_cooling_w: 0.0, compressor_power_w: 0.0, condensate_kg_s: 0.0 };
    }

    let h_in = moist_air_enthalpy_j_per_kg(inlet.temperature_c, inlet.humidity_ratio);
    let t_apparatus_dew = apparatus_dew_point_c(inlet.temperature_c, inlet.humidity_ratio, inlet.pressure_pa);
    let t_adp = t_apparatus_dew;

    let (q_max, cop, shr) = match coil {
        CoolingCoil::ChilledWater { ua_w_per_k, water_inlet_c, .. } => {
            let q = ua_w_per_k * (inlet.temperature_c - water_inlet_c).max(0.0);
            (q, 5.0, 0.75)
        }
        CoolingCoil::DxSingleSpeed { rated_capacity_w, rated_shr, cop_curve } => {
            let plr = (load_w / rated_capacity_w.max(1.0)).clamp(0.0, 1.0);
            let cop = 3.5 * cop_curve.evaluate(plr).max(0.5);
            (*rated_capacity_w, cop, *rated_shr)
        }
        CoolingCoil::DxMultiSpeed { stages } => {
            let mut remaining = load_w;
            let mut cap = 0.0;
            let mut cop_sum = 0.0;
            let mut shr_sum = 0.0;
            let mut n = 0.0;
            for stage in stages {
                if remaining <= 0.0 {
                    break;
                }
                let q = remaining.min(stage.capacity_w);
                cap += q;
                cop_sum += stage.cop * q;
                shr_sum += stage.shr * q;
                n += q;
                remaining -= q;
            }
            let cop = if n > 0.0 { cop_sum / n } else { 3.0 };
            let shr = if n > 0.0 { shr_sum / n } else { 0.7 };
            (cap, cop, shr)
        }
        CoolingCoil::DxVariableSpeed { rated_capacity_w, rated_cop, cop_curve } => {
            let plr = (load_w / rated_capacity_w.max(1.0)).clamp(0.0, 1.0);
            let cop = rated_cop * cop_curve.evaluate(plr).max(0.5);
            (*rated_capacity_w, cop, 0.72)
        }
    };

    let q_total = load_w.min(q_max);
    let q_sensible = q_total * shr;
    let q_latent = q_total - q_sensible;

    let t_saturated = t_adp;
    let w_sat = saturation_humidity_ratio(t_saturated, inlet.pressure_pa);
    let h_sat = moist_air_enthalpy_j_per_kg(t_saturated, w_sat);
    let h_out_ideal = h_in - q_total / m_dot;
    let _h_out = bf * h_in + (1.0 - bf) * h_out_ideal;
    let t_out_ideal = inlet.temperature_c - q_sensible / (m_dot * CP_DRY_AIR);
    let t_out = bf * inlet.temperature_c + (1.0 - bf) * t_out_ideal;
    let w_out_ideal = inlet.humidity_ratio - q_latent / (m_dot * latent_heat_vaporization(t_out));
    let w_out = (bf * inlet.humidity_ratio + (1.0 - bf) * w_out_ideal).max(0.0);

    let condensate = (inlet.humidity_ratio - w_out).max(0.0) * m_dot;
    let compressor_power = q_total / cop.max(0.5);

    let _ = h_sat;

    CoolingCoilOutput {
        outlet: CoilAirState { temperature_c: t_out, humidity_ratio: w_out, ..*inlet },
        total_cooling_w: q_total,
        sensible_cooling_w: q_sensible,
        latent_cooling_w: q_latent,
        compressor_power_w: compressor_power,
        condensate_kg_s: condensate,
    }
}

fn apparatus_dew_point_c(t_db: f64, w: f64, p_atm: f64) -> f64 {
    let p_ws = saturation_pressure_pa(t_db);
    let p_w = w * p_atm / (0.621_945 + w);
    let rh = (p_w / p_ws).clamp(0.01, 1.0);
    t_db - (1.0 - rh) * 5.0
}

fn saturation_humidity_ratio(t_c: f64, p_atm: f64) -> f64 {
    let p_ws = saturation_pressure_pa(t_c);
    0.621_945 * p_ws / (p_atm - p_ws).max(1.0)
}
// #endregion 🔖️CoolingOutput

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn electric_heating_raises_temperature() {
        let coil = HeatingCoil::Electric { capacity_w: 10_000.0, efficiency: 1.0 };
        let inlet = CoilAirState { temperature_c: 15.0, humidity_ratio: 0.008, mass_flow_kg_s: 0.5, pressure_pa: 101_325.0 };
        let out = heating_coil_output_w(&coil, &inlet, 5000.0);
        assert!(out.outlet.temperature_c > inlet.temperature_c);
        assert!((out.total_heating_w - 5000.0).abs() < 1.0);
    }

    #[test]
    fn dx_cooling_removes_sensible_and_latent() {
        let coil = CoolingCoil::DxSingleSpeed { rated_capacity_w: 15_000.0, rated_shr: 0.75, cop_curve: PerformanceCurve::Constant(1.0) };
        let inlet = CoilAirState { temperature_c: 28.0, humidity_ratio: 0.012, mass_flow_kg_s: 0.6, pressure_pa: 101_325.0 };
        let out = cooling_coil_output_w(&coil, &inlet, 10_000.0, 0.1);
        assert!(out.outlet.temperature_c < inlet.temperature_c);
        assert!(out.sensible_cooling_w > 0.0);
        assert!(out.latent_cooling_w > 0.0);
        assert!(out.compressor_power_w > 0.0);
    }

    #[test]
    fn bypass_factor_reduces_effect() {
        let coil = CoolingCoil::DxSingleSpeed { rated_capacity_w: 15_000.0, rated_shr: 0.8, cop_curve: PerformanceCurve::Constant(1.0) };
        let inlet = CoilAirState { temperature_c: 30.0, humidity_ratio: 0.014, mass_flow_kg_s: 0.5, pressure_pa: 101_325.0 };
        let out_low_bf = cooling_coil_output_w(&coil, &inlet, 8000.0, 0.05);
        let out_high_bf = cooling_coil_output_w(&coil, &inlet, 8000.0, 0.4);
        assert!(out_low_bf.outlet.temperature_c < out_high_bf.outlet.temperature_c);
    }
}
