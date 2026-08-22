//! 🎯️ Ideal loads air system: unlimited or capacity-limited zone conditioning.

use crate::props::moist_air_enthalpy_j_per_kg;
use crate::units::{CP_DRY_AIR, H_FG_0C, RHO_AIR_REF};
use serde::{Deserialize, Serialize};

// #region 🔖️IdealLoads
/// 🎯️ Ideal loads physics configuration (distinct from [`crate::model::IdealLoadsSystem`] entity).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdealLoadsConfig {
    pub max_heating_supply_air_temp_c: f64,
    pub min_cooling_supply_air_temp_c: f64,
    pub max_heating_capacity_w: Option<f64>,
    pub max_cooling_capacity_w: Option<f64>,
    pub outdoor_air_per_person_m3_s: f64,
    pub outdoor_air_per_area_m3_s_m2: f64,
}

impl Default for IdealLoadsConfig {
    fn default() -> Self {
        Self { max_heating_supply_air_temp_c: 50.0, min_cooling_supply_air_temp_c: 13.0, max_heating_capacity_w: None, max_cooling_capacity_w: None, outdoor_air_per_person_m3_s: 0.009_44, outdoor_air_per_area_m3_s_m2: 0.0 }
    }
}

/// 🌬️ Economizer control mode.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum EconomizerControl {
    #[default]
    None,
    DifferentialDryBulb,
    DifferentialEnthalpy,
    FixedDryBulb {
        lockout_c: f64,
    },
}

/// 💧️ Humidity control mode.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum HumidityControl {
    #[default]
    None,
    HumidifyAndDehumidify,
    HumidifyOnly,
    DehumidifyOnly,
}

/// 📥️ Zone demand and boundary conditions for ideal loads.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdealLoadsInput {
    pub zone_temp_c: f64,
    pub zone_humidity_ratio: f64,
    pub outdoor_temp_c: f64,
    pub outdoor_humidity_ratio: f64,
    pub heating_setpoint_c: f64,
    pub cooling_setpoint_c: f64,
    pub zone_heating_demand_w: f64,
    pub zone_cooling_demand_w: f64,
    pub occupancy: f64,
    pub floor_area_m2: f64,
}

/// Alias for callers that still use the request naming.
pub type IdealLoadsRequest = IdealLoadsInput;

/// 📤️ Ideal loads delivery result per zone timestep.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdealLoadsOutput {
    pub sensible_heating_w: f64,
    pub sensible_cooling_w: f64,
    pub latent_heating_w: f64,
    pub latent_cooling_w: f64,
    pub sensible_delivered_w: f64,
    pub unmet_heating_w: f64,
    pub unmet_cooling_w: f64,
    pub outdoor_air_mass_flow_kg_s: f64,
    pub supply_temperature_c: f64,
    pub supply_humidity_ratio: f64,
    pub economizer_active: bool,
    pub humidification_kg_s: f64,
    pub dehumidification_kg_s: f64,
}
// #endregion 🔖️IdealLoads

// #region 🔖️Deliver
/// 🎯️ Deliver ideal heating/cooling to meet zone setpoints and demands.
///
/// Argument order matches the simulation kernel: `(input, system)`.
pub fn ideal_loads_deliver(input: &IdealLoadsInput, system: &IdealLoadsConfig) -> IdealLoadsOutput {
    ideal_loads_deliver_with_controls(input, system, EconomizerControl::None, HumidityControl::None)
}

/// 🎯️ Ideal loads with explicit economizer and humidity controls.
pub fn ideal_loads_deliver_with_controls(input: &IdealLoadsInput, system: &IdealLoadsConfig, economizer: EconomizerControl, humidity_control: HumidityControl) -> IdealLoadsOutput {
    let oa_vol = system.outdoor_air_per_person_m3_s * input.occupancy + system.outdoor_air_per_area_m3_s_m2 * input.floor_area_m2;
    let economizer_active = economizer_allows_oa(economizer, input);
    let oa_m_dot = oa_vol.max(0.0) * RHO_AIR_REF;

    let heat_demand = input.zone_heating_demand_w.max(0.0);
    let cool_demand = input.zone_cooling_demand_w.max(0.0);

    let sensible_heating = apply_capacity(heat_demand, system.max_heating_capacity_w);
    let sensible_cooling = apply_capacity(cool_demand, system.max_cooling_capacity_w);
    let unmet_heating_w = (heat_demand - sensible_heating).max(0.0);
    let unmet_cooling_w = (cool_demand - sensible_cooling).max(0.0);

    let supply_temperature_c = if sensible_heating > 0.0 {
        system.max_heating_supply_air_temp_c
    } else if sensible_cooling > 0.0 {
        system.min_cooling_supply_air_temp_c
    } else {
        input.zone_temp_c
    };

    let mut latent_heating = 0.0;
    let mut latent_cooling = 0.0;
    let mut humidification = 0.0_f64;
    let mut dehumidification = 0.0_f64;
    let mut supply_w = input.zone_humidity_ratio;

    match humidity_control {
        HumidityControl::HumidifyOnly | HumidityControl::HumidifyAndDehumidify if input.zone_humidity_ratio < 0.008 => {
            humidification = 0.001 * oa_m_dot.max(0.01);
            latent_heating = humidification * H_FG_0C;
            supply_w = 0.008;
        }
        _ => {}
    }
    match humidity_control {
        HumidityControl::DehumidifyOnly | HumidityControl::HumidifyAndDehumidify if input.zone_humidity_ratio > 0.012 => {
            dehumidification = 0.001 * oa_m_dot.max(0.01);
            latent_cooling = dehumidification * H_FG_0C;
            supply_w = 0.012;
        }
        _ => {}
    }

    if economizer_active && sensible_cooling > 0.0 {
        let _free_cool = oa_m_dot * CP_DRY_AIR * (input.zone_temp_c - input.outdoor_temp_c).max(0.0);
    }

    IdealLoadsOutput {
        sensible_heating_w: sensible_heating,
        sensible_cooling_w: sensible_cooling,
        latent_heating_w: latent_heating,
        latent_cooling_w: latent_cooling,
        sensible_delivered_w: sensible_heating - sensible_cooling,
        unmet_heating_w,
        unmet_cooling_w,
        outdoor_air_mass_flow_kg_s: oa_m_dot,
        supply_temperature_c,
        supply_humidity_ratio: supply_w,
        economizer_active,
        humidification_kg_s: humidification,
        dehumidification_kg_s: dehumidification,
    }
}

fn apply_capacity(load: f64, cap: Option<f64>) -> f64 {
    match cap {
        Some(c) => load.min(c),
        None => load,
    }
}

fn economizer_allows_oa(economizer: EconomizerControl, input: &IdealLoadsInput) -> bool {
    match economizer {
        EconomizerControl::None => false,
        EconomizerControl::DifferentialDryBulb => input.outdoor_temp_c < input.zone_temp_c,
        EconomizerControl::DifferentialEnthalpy => {
            let h_oa = moist_air_enthalpy_j_per_kg(input.outdoor_temp_c, input.outdoor_humidity_ratio);
            let h_zone = moist_air_enthalpy_j_per_kg(input.zone_temp_c, input.zone_humidity_ratio);
            h_oa < h_zone
        }
        EconomizerControl::FixedDryBulb { lockout_c } => input.outdoor_temp_c < lockout_c,
    }
}
// #endregion 🔖️Deliver

#[cfg(test)]
mod tests {
    use super::*;

    fn unlimited_system() -> IdealLoadsConfig {
        IdealLoadsConfig { max_heating_supply_air_temp_c: 50.0, min_cooling_supply_air_temp_c: 13.0, max_heating_capacity_w: None, max_cooling_capacity_w: None, outdoor_air_per_person_m3_s: 0.01, outdoor_air_per_area_m3_s_m2: 0.0 }
    }

    #[test]
    fn heating_meets_demand() {
        let system = unlimited_system();
        let input = IdealLoadsInput {
            zone_temp_c: 18.0,
            zone_humidity_ratio: 0.008,
            outdoor_temp_c: 5.0,
            outdoor_humidity_ratio: 0.004,
            heating_setpoint_c: 21.0,
            cooling_setpoint_c: 24.0,
            zone_heating_demand_w: 3000.0,
            zone_cooling_demand_w: 0.0,
            occupancy: 2.0,
            floor_area_m2: 50.0,
        };
        let out = ideal_loads_deliver(&input, &system);
        assert!((out.sensible_heating_w - 3000.0).abs() < 1e-6);
        assert_eq!(out.unmet_heating_w, 0.0);
        assert!(out.sensible_delivered_w > 0.0);
    }

    #[test]
    fn capacity_limits_cooling() {
        let system = IdealLoadsConfig { max_cooling_capacity_w: Some(1000.0), ..unlimited_system() };
        let input = IdealLoadsInput {
            zone_temp_c: 30.0,
            zone_humidity_ratio: 0.01,
            outdoor_temp_c: 35.0,
            outdoor_humidity_ratio: 0.015,
            heating_setpoint_c: 21.0,
            cooling_setpoint_c: 24.0,
            zone_heating_demand_w: 0.0,
            zone_cooling_demand_w: 5000.0,
            occupancy: 1.0,
            floor_area_m2: 40.0,
        };
        let out = ideal_loads_deliver(&input, &system);
        assert!((out.sensible_cooling_w - 1000.0).abs() < 1e-6);
        assert!((out.unmet_cooling_w - 4000.0).abs() < 1e-6);
    }

    #[test]
    fn economizer_active_when_oa_cooler() {
        let system = unlimited_system();
        let input = IdealLoadsInput {
            zone_temp_c: 25.0,
            zone_humidity_ratio: 0.01,
            outdoor_temp_c: 15.0,
            outdoor_humidity_ratio: 0.006,
            heating_setpoint_c: 21.0,
            cooling_setpoint_c: 24.0,
            zone_heating_demand_w: 0.0,
            zone_cooling_demand_w: 2000.0,
            occupancy: 1.0,
            floor_area_m2: 30.0,
        };
        let out = ideal_loads_deliver_with_controls(&input, &system, EconomizerControl::DifferentialDryBulb, HumidityControl::None);
        assert!(out.economizer_active);
    }
}
