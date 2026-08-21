//! 🌬️ Air terminals: VAV, CAV, reheat, fan-powered, and dual-duct.

use crate::coils::{heating_coil_output_w, CoilAirState, HeatingCoil};
use crate::fans::{fan_mass_flow_kg_s, fan_operating_point, fan_power_w, Fan};
use crate::units::RHO_AIR_REF;
use serde::{Deserialize, Serialize};

// #region 🔖️AirTerminal
/// 🌬️ Zone air terminal unit types.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AirTerminal {
    Cav { max_flow_m3_s: f64 },
    Vav { min_flow_m3_s: f64, max_flow_m3_s: f64, reheat: Option<HeatingCoil> },
    VavReheat { min_flow_m3_s: f64, max_flow_m3_s: f64, reheat: HeatingCoil },
    FanPowered { primary_max_m3_s: f64, fan: Box<Fan>, parallel_fan: bool },
    DualDuct { hot_max_m3_s: f64, cold_max_m3_s: f64, mixing_damper: f64 },
}

/// 📥️ Terminal inlet air and zone load request.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct TerminalRequest {
    pub supply_temperature_c: f64,
    pub supply_humidity_ratio: f64,
    pub zone_temperature_c: f64,
    pub zone_humidity_ratio: f64,
    pub heating_load_w: f64,
    pub cooling_load_w: f64,
    pub pressure_pa: f64,
    pub damper_position: f64,
    pub hot_duct_temp_c: f64,
    pub cold_duct_temp_c: f64,
}

/// 📤️ Terminal outlet air delivered to zone.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct TerminalOutput {
    pub discharge_temperature_c: f64,
    pub discharge_humidity_ratio: f64,
    pub mass_flow_kg_s: f64,
    pub primary_mass_flow_kg_s: f64,
    pub reheat_w: f64,
    pub fan_power_w: f64,
    pub damper_position: f64,
}
// #endregion 🔖️AirTerminal

// #region 🔖️Simulate
impl AirTerminal {
    /// 🌬️ Simulate terminal unit for one zone timestep.
    pub fn simulate(&self, request: &TerminalRequest) -> TerminalOutput {
        match self {
            AirTerminal::Cav { max_flow_m3_s } => {
                let m_dot = fan_mass_flow_kg_s(*max_flow_m3_s, RHO_AIR_REF);
                TerminalOutput {
                    discharge_temperature_c: request.supply_temperature_c,
                    discharge_humidity_ratio: request.supply_humidity_ratio,
                    mass_flow_kg_s: m_dot,
                    primary_mass_flow_kg_s: m_dot,
                    reheat_w: 0.0,
                    fan_power_w: 0.0,
                    damper_position: 1.0,
                }
            }
            AirTerminal::Vav { min_flow_m3_s, max_flow_m3_s, reheat } => {
                let frac = request.damper_position.clamp(0.0, 1.0);
                let flow = min_flow_m3_s + frac * (max_flow_m3_s - min_flow_m3_s);
                let m_dot = fan_mass_flow_kg_s(flow, RHO_AIR_REF);
                let inlet = CoilAirState { temperature_c: request.supply_temperature_c, humidity_ratio: request.supply_humidity_ratio, mass_flow_kg_s: m_dot, pressure_pa: request.pressure_pa };
                let reheat_out = reheat.as_ref().map_or(crate::coils::HeatingCoilOutput { outlet: inlet, total_heating_w: 0.0, gas_consumption_w: 0.0, water_heat_removal_w: 0.0 }, |c| heating_coil_output_w(c, &inlet, request.heating_load_w));
                TerminalOutput {
                    discharge_temperature_c: reheat_out.outlet.temperature_c,
                    discharge_humidity_ratio: reheat_out.outlet.humidity_ratio,
                    mass_flow_kg_s: m_dot,
                    primary_mass_flow_kg_s: m_dot,
                    reheat_w: reheat_out.total_heating_w,
                    fan_power_w: 0.0,
                    damper_position: frac,
                }
            }
            AirTerminal::VavReheat { min_flow_m3_s, max_flow_m3_s, reheat } => {
                let frac = request.damper_position.clamp(0.0, 1.0);
                let flow = min_flow_m3_s + frac * (max_flow_m3_s - min_flow_m3_s);
                let m_dot = fan_mass_flow_kg_s(flow, RHO_AIR_REF);
                let inlet = CoilAirState { temperature_c: request.supply_temperature_c, humidity_ratio: request.supply_humidity_ratio, mass_flow_kg_s: m_dot, pressure_pa: request.pressure_pa };
                let reheat_out = heating_coil_output_w(reheat, &inlet, request.heating_load_w);
                TerminalOutput {
                    discharge_temperature_c: reheat_out.outlet.temperature_c,
                    discharge_humidity_ratio: reheat_out.outlet.humidity_ratio,
                    mass_flow_kg_s: m_dot,
                    primary_mass_flow_kg_s: m_dot,
                    reheat_w: reheat_out.total_heating_w,
                    fan_power_w: 0.0,
                    damper_position: frac,
                }
            }
            AirTerminal::FanPowered { primary_max_m3_s, fan, parallel_fan } => {
                let frac = request.damper_position.clamp(0.0, 1.0);
                let primary_flow = primary_max_m3_s * frac;
                let induced_flow = if *parallel_fan { primary_flow * 0.5 } else { primary_flow * 0.3 };
                let total_flow = primary_flow + induced_flow;
                let m_dot = fan_mass_flow_kg_s(total_flow, RHO_AIR_REF);
                let t_mix = if *parallel_fan { (primary_flow * request.supply_temperature_c + induced_flow * request.zone_temperature_c) / total_flow.max(1e-6) } else { request.supply_temperature_c };
                let operating_point = fan_operating_point(fan, induced_flow, 150.0);
                TerminalOutput {
                    discharge_temperature_c: t_mix,
                    discharge_humidity_ratio: request.supply_humidity_ratio,
                    mass_flow_kg_s: m_dot,
                    primary_mass_flow_kg_s: fan_mass_flow_kg_s(primary_flow, RHO_AIR_REF),
                    reheat_w: 0.0,
                    fan_power_w: fan_power_w(fan, &operating_point),
                    damper_position: frac,
                }
            }
            AirTerminal::DualDuct { hot_max_m3_s, cold_max_m3_s, mixing_damper } => {
                let mix = mixing_damper.clamp(0.0, 1.0);
                let hot_flow = hot_max_m3_s * mix;
                let cold_flow = cold_max_m3_s * (1.0 - mix);
                let total_flow = hot_flow + cold_flow;
                let m_dot = fan_mass_flow_kg_s(total_flow, RHO_AIR_REF);
                let t_out = if total_flow > 1e-9 { (hot_flow * request.hot_duct_temp_c + cold_flow * request.cold_duct_temp_c) / total_flow } else { request.zone_temperature_c };
                TerminalOutput { discharge_temperature_c: t_out, discharge_humidity_ratio: request.supply_humidity_ratio, mass_flow_kg_s: m_dot, primary_mass_flow_kg_s: m_dot, reheat_w: 0.0, fan_power_w: 0.0, damper_position: mix }
            }
        }
    }
}
// #endregion 🔖️Simulate

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    fn cav_constant_flow() {
        let term = AirTerminal::Cav { max_flow_m3_s: 0.3 };
        let req = TerminalRequest {
            supply_temperature_c: 13.0,
            supply_humidity_ratio: 0.008,
            zone_temperature_c: 22.0,
            zone_humidity_ratio: 0.01,
            heating_load_w: 0.0,
            cooling_load_w: 2000.0,
            pressure_pa: 101_325.0,
            damper_position: 1.0,
            hot_duct_temp_c: 35.0,
            cold_duct_temp_c: 13.0,
        };
        let out = term.simulate(&req);
        assert!((out.mass_flow_kg_s - 0.36).abs() < 0.05);
        assert!((out.discharge_temperature_c - 13.0).abs() < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    fn vav_reheat_adds_heat() {
        let term = AirTerminal::VavReheat { min_flow_m3_s: 0.05, max_flow_m3_s: 0.4, reheat: HeatingCoil::Electric { capacity_w: 5000.0, efficiency: 1.0 } };
        let req = TerminalRequest {
            supply_temperature_c: 13.0,
            supply_humidity_ratio: 0.008,
            zone_temperature_c: 20.0,
            zone_humidity_ratio: 0.009,
            heating_load_w: 2000.0,
            cooling_load_w: 0.0,
            pressure_pa: 101_325.0,
            damper_position: 0.5,
            hot_duct_temp_c: 35.0,
            cold_duct_temp_c: 13.0,
        };
        let out = term.simulate(&req);
        assert!(out.reheat_w > 0.0);
        assert!(out.discharge_temperature_c > req.supply_temperature_c);
    }

    #[semio_framework_async_macros::async_test]
    fn dual_duct_mixes_temperatures() {
        let term = AirTerminal::DualDuct { hot_max_m3_s: 0.2, cold_max_m3_s: 0.3, mixing_damper: 0.5 };
        let req = TerminalRequest {
            supply_temperature_c: 15.0,
            supply_humidity_ratio: 0.009,
            zone_temperature_c: 22.0,
            zone_humidity_ratio: 0.01,
            heating_load_w: 0.0,
            cooling_load_w: 0.0,
            pressure_pa: 101_325.0,
            damper_position: 0.5,
            hot_duct_temp_c: 40.0,
            cold_duct_temp_c: 12.0,
        };
        let out = term.simulate(&req);
        assert!(out.discharge_temperature_c > req.cold_duct_temp_c);
        assert!(out.discharge_temperature_c < req.hot_duct_temp_c);
    }
}
