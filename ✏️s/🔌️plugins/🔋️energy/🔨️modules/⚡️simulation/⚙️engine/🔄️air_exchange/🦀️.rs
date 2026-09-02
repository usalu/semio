//! 💨️ Infiltration, ventilation, interzone mixing, and hybrid air exchange controls.

use crate::props::{latent_heat_vaporization, moist_air_density, moist_air_enthalpy_j_per_kg};
use crate::units::{CP_DRY_AIR, GRAVITY, RHO_AIR_REF};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

// #region 🔖️InfiltrationMethod
/// 🚪️ Infiltration flow calculation method.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub enum InfiltrationMethod {
    ScheduledAch,
    PerExteriorArea,
    EffectiveLeakageArea,
    WindAndStack,
}
// #endregion 🔖️InfiltrationMethod

// #region 🔖️InfiltrationSpec
/// 💨️ Infiltration model parameters (EnergyPlus-style wind/stack coefficients).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct InfiltrationSpec {
    pub method: InfiltrationMethod,
    pub schedule_factor: f64,
    pub ach: f64,
    pub flow_per_exterior_area_m3_s_m2: f64,
    pub effective_leakage_area_m2: f64,
    pub discharge_coefficient: f64,
    pub constant_coefficient: f64,
    pub temperature_coefficient: f64,
    pub velocity_coefficient: f64,
    pub velocity_squared_coefficient: f64,
    pub stack_height_m: f64,
}
// #endregion 🔖️InfiltrationSpec

// #region 🔖️VentilationSpec
/// 🌬️ Mechanical ventilation specification.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct VentilationSpec {
    pub design_flow_m3_s: f64,
    pub schedule_factor: f64,
    pub heat_recovery_effectiveness: f64,
    pub fan_heat_gain_w: f64,
    pub supply_temp_c: Option<f64>,
}
// #endregion 🔖️VentilationSpec

// #region 🔖️InterzoneMixing
/// ↔ Interzone air mixing between adjacent zones.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct InterzoneMixing {
    pub flow_m3_s: f64,
    pub schedule_factor: f64,
}
// #endregion 🔖️InterzoneMixing

// #region 🔖️HybridControl
/// 🎛️ Hybrid ventilation control: natural when conditions allow, mechanical otherwise.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct HybridVentilationControl {
    pub outdoor_temp_min_c: f64,
    pub outdoor_temp_max_c: f64,
    pub max_wind_speed_m_s: f64,
    pub natural_ach: f64,
    pub mechanical_backup: bool,
}
// #endregion 🔖️HybridControl

// #region 🔖️InfiltrationFlow
/// 💨️ Infiltration volumetric flow [m³/s].
pub fn infiltration_flow_m3_s(spec: &InfiltrationSpec, zone_volume_m3: f64, exterior_area_m2: f64, outdoor_temp_c: f64, zone_temp_c: f64, wind_speed_m_s: f64, p_atm: f64) -> f64 {
    let sf = spec.schedule_factor.clamp(0.0, 1.0);
    match spec.method {
        InfiltrationMethod::ScheduledAch => sf * spec.ach * zone_volume_m3 / 3600.0,
        InfiltrationMethod::PerExteriorArea => sf * spec.flow_per_exterior_area_m3_s_m2 * exterior_area_m2,
        InfiltrationMethod::EffectiveLeakageArea => {
            let rho = moist_air_density(outdoor_temp_c, 0.008, p_atm);
            let delta_p = wind_stack_pressure_pa(spec.stack_height_m, outdoor_temp_c, zone_temp_c, wind_speed_m_s, spec.constant_coefficient, spec.temperature_coefficient, spec.velocity_coefficient, spec.velocity_squared_coefficient);
            sf * spec.discharge_coefficient * spec.effective_leakage_area_m2 * (2.0 * delta_p.max(0.0) / rho).sqrt()
        }
        InfiltrationMethod::WindAndStack => {
            let base = sf * spec.flow_per_exterior_area_m3_s_m2 * exterior_area_m2;
            let delta_t = (outdoor_temp_c - zone_temp_c).abs();
            let wind_factor = 1.0 + spec.velocity_coefficient * wind_speed_m_s + spec.velocity_squared_coefficient * wind_speed_m_s * wind_speed_m_s;
            let temp_factor = 1.0 + spec.temperature_coefficient * delta_t;
            base * wind_factor * temp_factor + sf * spec.constant_coefficient
        }
    }
}

fn wind_stack_pressure_pa(height_m: f64, t_out_c: f64, t_zone_c: f64, wind_m_s: f64, c_const: f64, c_temp: f64, c_vel: f64, c_vel2: f64) -> f64 {
    let t_out_k = t_out_c + 273.15;
    let t_zone_k = t_zone_c + 273.15;
    let stack = RHO_AIR_REF * GRAVITY * height_m * (t_out_k - t_zone_k).abs() / t_zone_k.max(250.0);
    let wind = 0.5 * RHO_AIR_REF * (c_vel * wind_m_s + c_vel2 * wind_m_s * wind_m_s);
    c_const + c_temp * (t_out_c - t_zone_c).abs() + stack + wind
}
// #endregion 🔖️InfiltrationFlow

// #region 🔖️VentilationLoad
/// 🔥️ Ventilation sensible and latent loads [W].
pub fn ventilation_load_w(flow_m3_s: f64, t_zone_c: f64, w_zone: f64, t_out_c: f64, w_out: f64, p_atm: f64, heat_recovery_effectiveness: f64) -> (f64, f64) {
    if flow_m3_s <= 0.0 {
        return (0.0, 0.0);
    }
    let rho = moist_air_density(t_out_c, w_out, p_atm);
    let m_dot = rho * flow_m3_s;
    let _h_zone = moist_air_enthalpy_j_per_kg(t_zone_c, w_zone);
    let _h_out = moist_air_enthalpy_j_per_kg(t_out_c, w_out);
    let eps = heat_recovery_effectiveness.clamp(0.0, 1.0);
    let sensible = m_dot * CP_DRY_AIR * (t_out_c - t_zone_c) * (1.0 - eps);
    let h_fg = latent_heat_vaporization((t_zone_c + t_out_c) * 0.5);
    let latent = m_dot * (w_out - w_zone) * h_fg * (1.0 - eps);
    (sensible, latent)
}
// #endregion 🔖️VentilationLoad

// #region 🔖️Interzone
/// ↔ Sensible and latent exchange [W] from interzone mixing flow.
pub fn interzone_exchange_w(mixing: &InterzoneMixing, t_zone_c: f64, w_zone: f64, t_adjacent_c: f64, w_adjacent: f64, p_atm: f64) -> (f64, f64) {
    let flow = mixing.flow_m3_s * mixing.schedule_factor.clamp(0.0, 1.0);
    ventilation_load_w(flow, t_zone_c, w_zone, t_adjacent_c, w_adjacent, p_atm, 0.0)
}
// #endregion 🔖️Interzone

// #region 🔖️Hybrid
/// 🎛️ Hybrid ventilation flow [m³/s]: natural when outdoor conditions favorable.
pub fn hybrid_ventilation_flow_m3_s(control: &HybridVentilationControl, zone_volume_m3: f64, outdoor_temp_c: f64, wind_speed_m_s: f64, mechanical_flow_m3_s: f64) -> f64 {
    let natural_ok = outdoor_temp_c >= control.outdoor_temp_min_c && outdoor_temp_c <= control.outdoor_temp_max_c && wind_speed_m_s <= control.max_wind_speed_m_s;
    if natural_ok {
        control.natural_ach * zone_volume_m3 / 3600.0
    } else if control.mechanical_backup {
        mechanical_flow_m3_s
    } else {
        0.0
    }
}
// #endregion 🔖️Hybrid

// #region 🔖️AirExchangeResult
/// 📊️ Combined air exchange flows and loads for one zone.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct AirExchangeResult {
    pub infiltration_flow_m3_s: f64,
    pub ventilation_flow_m3_s: f64,
    pub infiltration_sensible_w: f64,
    pub infiltration_latent_w: f64,
    pub ventilation_sensible_w: f64,
    pub ventilation_latent_w: f64,
}

/// 💨️ Compute combined infiltration and ventilation for a zone timestep.
pub fn compute_air_exchange(infiltration: &InfiltrationSpec, ventilation: &VentilationSpec, zone_volume_m3: f64, exterior_area_m2: f64, t_zone_c: f64, w_zone: f64, t_out_c: f64, w_out: f64, wind_speed_m_s: f64, p_atm: f64) -> AirExchangeResult {
    let inf_flow = infiltration_flow_m3_s(infiltration, zone_volume_m3, exterior_area_m2, t_out_c, t_zone_c, wind_speed_m_s, p_atm);
    let vent_flow = ventilation.design_flow_m3_s * ventilation.schedule_factor.clamp(0.0, 1.0);
    let (inf_sens, inf_lat) = ventilation_load_w(inf_flow, t_zone_c, w_zone, t_out_c, w_out, p_atm, 0.0);
    let (vent_sens, vent_lat) = ventilation_load_w(vent_flow, t_zone_c, w_zone, t_out_c, w_out, p_atm, ventilation.heat_recovery_effectiveness);
    AirExchangeResult {
        infiltration_flow_m3_s: inf_flow,
        ventilation_flow_m3_s: vent_flow,
        infiltration_sensible_w: inf_sens,
        infiltration_latent_w: inf_lat,
        ventilation_sensible_w: vent_sens + ventilation.fan_heat_gain_w,
        ventilation_latent_w: vent_lat,
    }
}
// #endregion 🔖️AirExchangeResult

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::P_STD;

    #[test]
    fn ach_infiltration_scales_with_volume() {
        let spec = InfiltrationSpec {
            method: InfiltrationMethod::ScheduledAch,
            schedule_factor: 1.0,
            ach: 0.5,
            flow_per_exterior_area_m3_s_m2: 0.0,
            effective_leakage_area_m2: 0.0,
            discharge_coefficient: 0.6,
            constant_coefficient: 0.0,
            temperature_coefficient: 0.0,
            velocity_coefficient: 0.0,
            velocity_squared_coefficient: 0.0,
            stack_height_m: 3.0,
        };
        let flow = infiltration_flow_m3_s(&spec, 200.0, 50.0, 5.0, 22.0, 3.0, P_STD);
        assert!((flow - 200.0 * 0.5 / 3600.0).abs() < 1e-9);
    }

    #[test]
    fn ventilation_load_positive_when_outdoor_colder() {
        let (sens, _) = ventilation_load_w(0.1, 22.0, 0.009, 5.0, 0.004, P_STD, 0.0);
        assert!(sens < 0.0);
    }

    #[test]
    fn heat_recovery_reduces_load() {
        let (sens0, _) = ventilation_load_w(0.2, 22.0, 0.009, 5.0, 0.004, P_STD, 0.0);
        let (sens1, _) = ventilation_load_w(0.2, 22.0, 0.009, 5.0, 0.004, P_STD, 0.8);
        assert!(sens1.abs() < sens0.abs());
    }

    #[test]
    fn hybrid_uses_natural_when_favorable() {
        let ctrl = HybridVentilationControl { outdoor_temp_min_c: 10.0, outdoor_temp_max_c: 28.0, max_wind_speed_m_s: 5.0, natural_ach: 2.0, mechanical_backup: true };
        let flow = hybrid_ventilation_flow_m3_s(&ctrl, 300.0, 20.0, 2.0, 0.05);
        assert!((flow - 300.0 * 2.0 / 3600.0).abs() < 1e-9);
    }
}
