//! ♻️ Heat recovery: sensible/latent exchange via effectiveness-NTU with frost control.

use crate::props::moist_air_enthalpy_j_per_kg;
use crate::units::{CP_DRY_AIR, H_FG_0C};
use serde::{Deserialize, Serialize};

// #region 🔖️HeatRecovery
/// ♻️ Heat recovery ventilator configuration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HeatRecoveryUnit {
    pub hx_type: HeatExchangerType,
    pub sensible_effectiveness: f64,
    pub latent_effectiveness: f64,
    pub frost_control_temp_c: f64,
    pub defrost_power_w: f64,
}

/// 🔀️ Heat exchanger flow arrangement.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HeatExchangerType {
    CounterFlow,
    CrossFlow,
    ParallelFlow,
}

/// 📥️ Supply and exhaust airstreams at HX inlet.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct HxAirstream {
    pub temperature_c: f64,
    pub humidity_ratio: f64,
    pub mass_flow_kg_s: f64,
    pub pressure_pa: f64,
}

/// 📤️ Heat recovery exchange result.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct HeatRecoveryOutput {
    pub supply_out: HxAirstream,
    pub exhaust_out: HxAirstream,
    pub sensible_recovery_w: f64,
    pub latent_recovery_w: f64,
    pub total_recovery_w: f64,
    pub defrost_active: bool,
    pub defrost_power_w: f64,
    pub effectiveness_sensible: f64,
    pub effectiveness_latent: f64,
}
// #endregion 🔖️HeatRecovery

// #region 🔖️Ntu
/// 📐️ Effectiveness from NTU and capacity ratio (counter-flow approximation).
pub fn effectiveness_from_ntu(ntu: f64, cr: f64, hx_type: HeatExchangerType) -> f64 {
    let cr = cr.clamp(0.0, 1.0);
    let ntu = ntu.max(0.0);
    match hx_type {
        HeatExchangerType::CounterFlow => {
            if (cr - 1.0).abs() < 1e-6 {
                ntu / (1.0 + ntu)
            } else {
                (1.0 - (-ntu * (1.0 - cr)).exp()) / (1.0 - cr * (-ntu * (1.0 - cr)).exp())
            }
        }
        HeatExchangerType::CrossFlow => {
            let n = ntu * (1.0 - 0.22 * cr.ln().abs().min(2.0));
            n / (1.0 + n)
        }
        HeatExchangerType::ParallelFlow => (1.0 - (-ntu * (1.0 + cr)).exp()) / (1.0 + cr),
    }
}

/// 📐️ NTU from UA and minimum capacity rate.
pub fn ntu_from_ua(ua_w_per_k: f64, c_min: f64) -> f64 {
    if c_min < 1e-9 {
        return 0.0;
    }
    ua_w_per_k / c_min
}
// #endregion 🔖️Ntu

// #region 🔖️Exchange
/// ♻️ Sensible and latent heat recovery exchange [W].
pub fn heat_recovery_exchange_w(unit: &HeatRecoveryUnit, supply_in: &HxAirstream, exhaust_in: &HxAirstream) -> HeatRecoveryOutput {
    let m_sup = supply_in.mass_flow_kg_s.max(0.0);
    let m_exh = exhaust_in.mass_flow_kg_s.max(0.0);
    if m_sup < 1e-9 || m_exh < 1e-9 {
        return passthrough(unit, supply_in, exhaust_in);
    }

    let c_sup = m_sup * CP_DRY_AIR;
    let c_exh = m_exh * CP_DRY_AIR;
    let c_min = c_sup.min(c_exh);
    let cr = c_min / c_sup.max(c_exh).max(1e-9);

    let eps_s = unit.sensible_effectiveness.clamp(0.0, 0.95);
    let eps_l = unit.latent_effectiveness.clamp(0.0, 0.85);

    let t_diff = exhaust_in.temperature_c - supply_in.temperature_c;
    let q_sensible = eps_s * c_min * t_diff;

    let w_diff = exhaust_in.humidity_ratio - supply_in.humidity_ratio;
    let q_latent = eps_l * m_sup.min(m_exh) * w_diff * H_FG_0C;

    let mut defrost = false;
    let mut defrost_power = 0.0_f64;
    let mut eff_s = eps_s;
    let mut eff_l = eps_l;

    if supply_in.temperature_c < unit.frost_control_temp_c && exhaust_in.temperature_c > supply_in.temperature_c {
        defrost = true;
        defrost_power = unit.defrost_power_w;
        eff_s *= 0.5;
        eff_l *= 0.3;
    }

    let q_sensible_adj = eff_s * c_min * t_diff;
    let q_latent_adj = eff_l * m_sup.min(m_exh) * w_diff * H_FG_0C;

    let supply_t = supply_in.temperature_c + q_sensible_adj / c_sup;
    let exhaust_t = exhaust_in.temperature_c - q_sensible_adj / c_exh;
    let supply_w = supply_in.humidity_ratio + eff_l * w_diff * m_exh / m_sup;
    let exhaust_w = exhaust_in.humidity_ratio - eff_l * w_diff * m_sup / m_exh;

    let h_sup_in = moist_air_enthalpy_j_per_kg(supply_in.temperature_c, supply_in.humidity_ratio);
    let h_sup_out = moist_air_enthalpy_j_per_kg(supply_t, supply_w);
    let h_exh_in = moist_air_enthalpy_j_per_kg(exhaust_in.temperature_c, exhaust_in.humidity_ratio);
    let h_exh_out = moist_air_enthalpy_j_per_kg(exhaust_t, exhaust_w);
    let q_total = m_sup * (h_sup_out - h_sup_in) - m_exh * (h_exh_out - h_exh_in);

    let _ = (q_sensible, q_latent, cr);

    HeatRecoveryOutput {
        supply_out: HxAirstream { temperature_c: supply_t, humidity_ratio: supply_w.max(0.0), mass_flow_kg_s: m_sup, pressure_pa: supply_in.pressure_pa },
        exhaust_out: HxAirstream { temperature_c: exhaust_t, humidity_ratio: exhaust_w.max(0.0), mass_flow_kg_s: m_exh, pressure_pa: exhaust_in.pressure_pa },
        sensible_recovery_w: q_sensible_adj,
        latent_recovery_w: q_latent_adj,
        total_recovery_w: q_total,
        defrost_active: defrost,
        defrost_power_w: defrost_power,
        effectiveness_sensible: eff_s,
        effectiveness_latent: eff_l,
    }
}

fn passthrough(unit: &HeatRecoveryUnit, supply: &HxAirstream, exhaust: &HxAirstream) -> HeatRecoveryOutput {
    HeatRecoveryOutput {
        supply_out: *supply,
        exhaust_out: *exhaust,
        sensible_recovery_w: 0.0,
        latent_recovery_w: 0.0,
        total_recovery_w: 0.0,
        defrost_active: false,
        defrost_power_w: 0.0,
        effectiveness_sensible: unit.sensible_effectiveness,
        effectiveness_latent: unit.latent_effectiveness,
    }
}
// #endregion 🔖️Exchange

#[cfg(test)]
mod tests {
    use super::*;

    fn erv() -> HeatRecoveryUnit {
        HeatRecoveryUnit { hx_type: HeatExchangerType::CounterFlow, sensible_effectiveness: 0.75, latent_effectiveness: 0.6, frost_control_temp_c: -5.0, defrost_power_w: 200.0 }
    }

    #[test]
    fn winter_recovery_heats_supply() {
        let unit = erv();
        let supply = HxAirstream { temperature_c: 5.0, humidity_ratio: 0.004, mass_flow_kg_s: 0.3, pressure_pa: 101_325.0 };
        let exhaust = HxAirstream { temperature_c: 22.0, humidity_ratio: 0.009, mass_flow_kg_s: 0.3, pressure_pa: 101_325.0 };
        let out = heat_recovery_exchange_w(&unit, &supply, &exhaust);
        assert!(out.supply_out.temperature_c > supply.temperature_c);
        assert!(out.sensible_recovery_w > 0.0);
    }

    #[test]
    fn effectiveness_ntu_counterflow() {
        let eps = effectiveness_from_ntu(3.0, 0.5, HeatExchangerType::CounterFlow);
        assert!(eps > 0.5 && eps < 1.0);
    }

    #[test]
    fn frost_reduces_effectiveness() {
        let unit = erv();
        let supply = HxAirstream { temperature_c: -10.0, humidity_ratio: 0.002, mass_flow_kg_s: 0.2, pressure_pa: 101_325.0 };
        let exhaust = HxAirstream { temperature_c: 20.0, humidity_ratio: 0.008, mass_flow_kg_s: 0.2, pressure_pa: 101_325.0 };
        let out = heat_recovery_exchange_w(&unit, &supply, &exhaust);
        assert!(out.defrost_active);
        assert!(out.defrost_power_w > 0.0);
    }
}
