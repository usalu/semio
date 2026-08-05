//! ❄️ Refrigeration: display cases, walk-ins, compressor racks, condensers, secondary loops.

use crate::curves::PerformanceCurve;
use crate::props::{r410a_saturation_pressure_pa, r410a_saturation_temp_c};
use crate::units::P_STD;
use serde::{Deserialize, Serialize};

// #region 🔖️State
/// 🌡️ Refrigeration circuit state.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct RefrigerationState {
    pub evaporating_temperature_c: f64,
    pub condensing_temperature_c: f64,
    pub suction_superheat_k: f64,
    pub liquid_subcool_k: f64,
}

/// 📤️ Refrigeration timestep output.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RefrigerationOutput {
    pub cooling_power_w: f64,
    pub compressor_power_w: f64,
    pub condenser_heat_w: f64,
    pub mass_flow_kg_s: f64,
}
// #endregion 🔖️State

// #region 🔖️DisplayCase
/// 🛒️ Supermarket display case with anti-sweat and fan power.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DisplayCase {
    pub length_m: f64,
    pub design_cooling_w: f64,
    pub fan_power_w: f64,
    pub lighting_power_w: f64,
    pub anti_sweat_heater_w: f64,
    pub defrost_power_w: f64,
    pub defrost_fraction: f64,
    pub evaporating_temperature_c: f64,
}

impl DisplayCase {
    /// 🛒️ Display case total cooling and electrical load.
    pub fn simulate(&self, case_temperature_c: f64, ambient_c: f64, load_factor: f64) -> RefrigerationOutput {
        let lf = load_factor.clamp(0.0, 1.5);
        let conductance = self.design_cooling_w / (case_temperature_c - self.evaporating_temperature_c).max(1.0);
        let cooling_w = conductance * (case_temperature_c - self.evaporating_temperature_c) * lf;
        let defrost_w = self.defrost_power_w * self.defrost_fraction;
        let anti_sweat = if ambient_c > 18.0 { self.anti_sweat_heater_w } else { 0.0 };
        let cop = 2.5 + 0.05 * (self.evaporating_temperature_c + 10.0);
        let compressor_w = cooling_w / cop + self.fan_power_w + self.lighting_power_w + anti_sweat + defrost_w;
        RefrigerationOutput { cooling_power_w: cooling_w, compressor_power_w: compressor_w, condenser_heat_w: cooling_w + compressor_w * 0.85, mass_flow_kg_s: cooling_w / 150_000.0 }
    }
}
// #endregion 🔖️DisplayCase

// #region 🔖️WalkIn
/// 🚪️ Walk-in cooler or freezer box.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WalkIn {
    pub floor_area_m2: f64,
    pub wall_area_m2: f64,
    pub ua_w_per_k: f64,
    pub internal_gain_w: f64,
    pub design_box_temperature_c: f64,
    pub evaporating_temperature_c: f64,
    pub door_opening_fraction: f64,
}

impl WalkIn {
    /// 🚪️ Walk-in envelope and infiltration cooling load.
    pub fn simulate(&self, ambient_c: f64, humidity_factor: f64) -> RefrigerationOutput {
        let delta_t = (ambient_c - self.design_box_temperature_c).max(0.0);
        let envelope_w = self.ua_w_per_k * delta_t;
        let infil_w = 50.0 * self.door_opening_fraction * delta_t.powf(1.2) * humidity_factor;
        let cooling_w = envelope_w + self.internal_gain_w + infil_w;
        let cop = if self.design_box_temperature_c < -10.0 { 1.8 } else { 3.0 };
        let compressor_w = cooling_w / cop;
        RefrigerationOutput { cooling_power_w: cooling_w, compressor_power_w: compressor_w, condenser_heat_w: cooling_w + compressor_w, mass_flow_kg_s: cooling_w / 140_000.0 }
    }
}
// #endregion 🔖️WalkIn

// #region 🔖️CompressorRack
/// 🏭️ Shared compressor rack serving multiple cases.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CompressorRack {
    pub rated_capacity_w: f64,
    pub compressor_count: u32,
    pub eir_curve: PerformanceCurve,
    pub eir_f_t_curve: PerformanceCurve,
    pub min_evaporating_c: f64,
    pub max_condensing_c: f64,
}

impl CompressorRack {
    /// 🏭️ Rack cooling capacity and power at floating suction/head pressure.
    pub fn simulate(&self, total_cooling_w: f64, evaporating_c: f64, condensing_c: f64) -> RefrigerationOutput {
        let load = total_cooling_w.min(self.rated_capacity_w * self.compressor_count as f64);
        if load <= 0.0 {
            return RefrigerationOutput::default();
        }
        let plr = self.eir_curve.part_load(load, self.rated_capacity_w);
        let eir = self.eir_curve.evaluate(plr) * self.eir_f_t_curve.evaluate_2d(evaporating_c, condensing_c).max(0.1);
        let compressor_w = load * eir;
        RefrigerationOutput { cooling_power_w: load, compressor_power_w: compressor_w, condenser_heat_w: load + compressor_w, mass_flow_kg_s: load / (150_000.0 * plr.max(0.2)) }
    }

    /// 🌡️ Floating head pressure from ambient dry-bulb.
    pub fn floating_head_c(&self, ambient_c: f64) -> f64 {
        (ambient_c + 10.0).clamp(25.0, self.max_condensing_c)
    }
}
// #endregion 🔖️CompressorRack

// #region 🔖️Condenser
/// 🌊️ Air-cooled or evaporative condenser rejecting rack heat.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RefrigerationCondenser {
    pub ua_w_per_k: f64,
    pub fan_power_w: f64,
    pub design_approach_k: f64,
    pub evaporative: bool,
}

impl RefrigerationCondenser {
    /// 🌊️ Condenser heat rejection and fan power.
    pub fn simulate(&self, heat_rejection_w: f64, ambient_c: f64, wet_bulb_c: f64) -> (f64, f64) {
        if heat_rejection_w <= 0.0 {
            return (ambient_c + self.design_approach_k, 0.0);
        }
        let sink_t = if self.evaporative { wet_bulb_c } else { ambient_c };
        let condensing_t = sink_t + self.design_approach_k;
        let actual_t = ambient_c + heat_rejection_w / self.ua_w_per_k.max(1.0);
        let fan_w = self.fan_power_w * (heat_rejection_w / 100_000.0).clamp(0.2, 1.0);
        (actual_t.max(condensing_t), fan_w)
    }
}
// #endregion 🔖️Condenser

// #region 🔖️SecondaryLoop
/// 🧊️ Glycol secondary loop for remote display cases.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecondaryLoop {
    pub pump_power_w: f64,
    pub pipe_ua_w_per_k: f64,
    pub supply_temperature_c: f64,
    pub return_temperature_c: f64,
    pub mass_flow_kg_s: f64,
    pub fluid_cp: f64,
}

impl SecondaryLoop {
    /// 🧊️ Secondary loop pump and heat pickup from cases.
    pub fn simulate(&self, case_load_w: f64, ambient_c: f64) -> (f64, f64, f64) {
        let pipe_loss_w = self.pipe_ua_w_per_k * (self.supply_temperature_c - ambient_c).max(0.0);
        let fluid_cooling = case_load_w + pipe_loss_w;
        let delta_t = fluid_cooling / (self.mass_flow_kg_s.max(0.01) * self.fluid_cp);
        let new_return = self.supply_temperature_c + delta_t;
        let pump_w = self.pump_power_w * (case_load_w / 50_000.0).clamp(0.3, 1.0);
        (fluid_cooling, new_return, pump_w)
    }
}
// #endregion 🔖️SecondaryLoop

// #region 🔖️Circuit
/// ❄️ Full refrigeration circuit pressure-temperature check.
pub fn refrigeration_state_from_pressures(suction_pa: f64, discharge_pa: f64) -> RefrigerationState {
    RefrigerationState { evaporating_temperature_c: r410a_saturation_temp_c(suction_pa.max(P_STD * 0.3)), condensing_temperature_c: r410a_saturation_temp_c(discharge_pa.max(P_STD)), suction_superheat_k: 5.0, liquid_subcool_k: 3.0 }
}

/// ❄️ Estimate suction pressure from evaporating temperature.
pub fn evaporating_pressure_pa(t_evap_c: f64) -> f64 {
    r410a_saturation_pressure_pa(t_evap_c)
}
// #endregion 🔖️Circuit

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curves::PerformanceCurve;

    #[test]
    fn display_case_cooling_positive() {
        let case = DisplayCase { length_m: 3.0, design_cooling_w: 2000.0, fan_power_w: 150.0, lighting_power_w: 200.0, anti_sweat_heater_w: 100.0, defrost_power_w: 500.0, defrost_fraction: 0.05, evaporating_temperature_c: -8.0 };
        let out = case.simulate(-2.0, 22.0, 1.0);
        assert!(out.cooling_power_w > 0.0);
        assert!(out.compressor_power_w > out.cooling_power_w * 0.3);
    }

    #[test]
    fn walk_in_freezer_higher_compressor_ratio() {
        let cooler = WalkIn { floor_area_m2: 20.0, wall_area_m2: 60.0, ua_w_per_k: 30.0, internal_gain_w: 500.0, design_box_temperature_c: 2.0, evaporating_temperature_c: -5.0, door_opening_fraction: 0.1 };
        let freezer = WalkIn { design_box_temperature_c: -20.0, evaporating_temperature_c: -28.0, ..cooler };
        let cool_out = cooler.simulate(25.0, 1.0);
        let freeze_out = freezer.simulate(25.0, 1.0);
        assert!(freeze_out.compressor_power_w / freeze_out.cooling_power_w > cool_out.compressor_power_w / cool_out.cooling_power_w);
    }

    #[test]
    fn compressor_rack_part_load() {
        let rack = CompressorRack { rated_capacity_w: 100_000.0, compressor_count: 2, eir_curve: PerformanceCurve::Constant(0.35), eir_f_t_curve: PerformanceCurve::Constant(1.0), min_evaporating_c: -15.0, max_condensing_c: 45.0 };
        let out = rack.simulate(80_000.0, -8.0, 35.0);
        assert!(out.cooling_power_w > 70_000.0);
        assert!(out.condenser_heat_w > out.cooling_power_w);
    }

    #[test]
    fn condenser_approach_temperature() {
        let cond = RefrigerationCondenser { ua_w_per_k: 5000.0, fan_power_w: 3000.0, design_approach_k: 8.0, evaporative: false };
        let (t_cond, fan_w) = cond.simulate(150_000.0, 30.0, 22.0);
        assert!(t_cond > 30.0);
        assert!(fan_w > 0.0);
    }

    #[test]
    fn secondary_loop_return_rises_with_load() {
        let loop_sys = SecondaryLoop { pump_power_w: 800.0, pipe_ua_w_per_k: 20.0, supply_temperature_c: -5.0, return_temperature_c: -3.0, mass_flow_kg_s: 5.0, fluid_cp: 3500.0 };
        let (_, return_t, _) = loop_sys.simulate(30_000.0, 20.0);
        assert!(return_t > loop_sys.supply_temperature_c);
    }

    #[test]
    fn saturation_pressure_increases_with_temperature() {
        let low = evaporating_pressure_pa(-10.0);
        let high = evaporating_pressure_pa(5.0);
        assert!(high > low);
    }
}
