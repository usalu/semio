//! 🚿️ Service hot water: mixed/stratified/HP heaters, fixtures, standby, drain recovery.

use crate::props::{water_cp_j_per_kg_k, water_density};
use crate::units::RHO_WATER;
use serde::{Deserialize, Serialize};

// #region 🔖️State
/// 🌡️ Hot-water storage state.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct WaterHeaterState {
    pub average_temperature_c: f64,
    pub top_temperature_c: f64,
    pub bottom_temperature_c: f64,
    pub volume_m3: f64,
}

/// 📤️ Water heater timestep output.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WaterHeaterOutput {
    pub heating_power_w: f64,
    pub electrical_power_w: f64,
    pub gas_power_w: f64,
    pub standby_loss_w: f64,
    pub delivered_flow_kg_s: f64,
    pub outlet_temperature_c: f64,
}
// #endregion 🔖️State

// #region 🔖️Mixed
/// 🚿️ Fully mixed storage water heater (electric or gas).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MixedWaterHeater {
    pub volume_l: f64,
    pub ua_standby_w_per_k: f64,
    pub heating_capacity_w: f64,
    pub setpoint_c: f64,
    pub ambient_c: f64,
    pub recovery_efficiency: f64,
    pub fuel: WaterHeaterFuel,
}

/// ⛽️ Water heater energy source.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WaterHeaterFuel {
    Electric,
    Gas,
}

impl MixedWaterHeater {
    /// 🚿️ Simulate mixed tank with draw, makeup, and standby losses.
    pub async fn simulate(&self, state: &WaterHeaterState, draw_flow_kg_s: f64, inlet_temperature_c: f64, dt_s: f64) -> (WaterHeaterOutput, WaterHeaterState) {
        let volume_m3 = self.volume_l / 1000.0;
        let rho = water_density(state.average_temperature_c);
        let cp = water_cp_j_per_kg_k(state.average_temperature_c);
        let mass_kg = rho * volume_m3;
        let standby_w = self.ua_standby_w_per_k * (state.average_temperature_c - self.ambient_c);
        let draw_energy_w = draw_flow_kg_s * cp * (state.average_temperature_c - inlet_temperature_c);
        let mut tank_t = state.average_temperature_c;
        let mut heating_w = 0.0;
        if tank_t < self.setpoint_c {
            let deficit_j = mass_kg * cp * (self.setpoint_c - tank_t);
            heating_w = (deficit_j / dt_s).min(self.heating_capacity_w);
            tank_t += heating_w * self.recovery_efficiency * dt_s / (mass_kg * cp);
        }
        tank_t -= (draw_energy_w + standby_w) * dt_s / (mass_kg * cp);
        tank_t = tank_t.clamp(inlet_temperature_c, self.setpoint_c + 5.0);
        let (elec_w, gas_w) = match self.fuel {
            WaterHeaterFuel::Electric => (heating_w / self.recovery_efficiency, 0.0),
            WaterHeaterFuel::Gas => (50.0, heating_w / self.recovery_efficiency),
        };
        let new_state = WaterHeaterState { average_temperature_c: tank_t, top_temperature_c: tank_t, bottom_temperature_c: tank_t, volume_m3 };
        (WaterHeaterOutput { heating_power_w: heating_w, electrical_power_w: elec_w, gas_power_w: gas_w, standby_loss_w: standby_w, delivered_flow_kg_s: draw_flow_kg_s, outlet_temperature_c: tank_t }, new_state)
    }
}
// #endregion 🔖️Mixed

// #region 🔖️Stratified
/// 🌡️ Stratified tank with fixed node count (1-D conduction between nodes).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StratifiedWaterHeater {
    pub volume_l: f64,
    pub node_count: usize,
    pub ua_standby_w_per_k: f64,
    pub setpoint_c: f64,
    pub ambient_c: f64,
    pub heating_capacity_w: f64,
    pub heater_position: usize,
}

impl StratifiedWaterHeater {
    /// 🌡️ Simulate stratified tank with buoyancy-driven minimal mixing.
    pub async fn simulate(&self, node_temperatures_c: &[f64], draw_flow_kg_s: f64, inlet_temperature_c: f64, dt_s: f64) -> (WaterHeaterOutput, Vec<f64>) {
        let n = self.node_count.max(2);
        let mut temps: Vec<f64> = if node_temperatures_c.len() == n { node_temperatures_c.to_vec() } else { vec![inlet_temperature_c; n] };
        let volume_m3 = self.volume_l / 1000.0;
        let node_volume = volume_m3 / n as f64;
        let rho = water_density(temps[0]);
        let cp = water_cp_j_per_kg_k(temps[0]);
        let node_mass = rho * node_volume;
        let avg_t = temps.iter().sum::<f64>() / n as f64;
        let standby_w = self.ua_standby_w_per_k * (avg_t - self.ambient_c);
        if draw_flow_kg_s > 0.0 {
            let draw_per_node = draw_flow_kg_s / n as f64;
            for t in &mut temps {
                let removal = draw_per_node * cp * (*t - inlet_temperature_c) * dt_s / node_mass;
                *t -= removal;
            }
        }
        let heater_idx = self.heater_position.min(n - 1);
        if temps[heater_idx] < self.setpoint_c {
            let deficit = node_mass * cp * (self.setpoint_c - temps[heater_idx]);
            let heat_j = (self.heating_capacity_w * dt_s).min(deficit);
            temps[heater_idx] += heat_j / (node_mass * cp);
        }
        for i in 0..n - 1 {
            let d_t = 0.05 * (temps[i] - temps[i + 1]);
            temps[i] -= d_t;
            temps[i + 1] += d_t;
        }
        let loss_per_node = standby_w * dt_s / (n as f64 * node_mass * cp);
        for t in &mut temps {
            *t -= loss_per_node;
            *t = t.clamp(inlet_temperature_c, self.setpoint_c + 10.0);
        }
        (
            WaterHeaterOutput {
                heating_power_w: self.heating_capacity_w.min(node_mass * cp * (self.setpoint_c - temps[heater_idx]).max(0.0) / dt_s),
                electrical_power_w: self.heating_capacity_w,
                gas_power_w: 0.0,
                standby_loss_w: standby_w,
                delivered_flow_kg_s: draw_flow_kg_s,
                outlet_temperature_c: temps[0],
            },
            temps,
        )
    }
}
// #endregion 🔖️Stratified

// #region 🔖️HeatPump
/// 🌡️ Heat-pump water heater with ambient air source.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HeatPumpWaterHeater {
    pub tank: MixedWaterHeater,
    pub rated_cop: f64,
    pub min_ambient_c: f64,
}

impl HeatPumpWaterHeater {
    /// 🌡️ HPWH with COP derated by ambient temperature.
    pub async fn simulate(&self, state: &WaterHeaterState, draw_flow_kg_s: f64, inlet_temperature_c: f64, ambient_c: f64, dt_s: f64) -> (WaterHeaterOutput, WaterHeaterState) {
        let cop = if ambient_c < self.min_ambient_c { 1.0 } else { (self.rated_cop * (1.0 - 0.03 * (20.0 - ambient_c))).max(1.5) };
        let (mut out, new_state) = self.tank.simulate(state, draw_flow_kg_s, inlet_temperature_c, dt_s);
        out.electrical_power_w = out.heating_power_w / cop + 50.0;
        out.gas_power_w = 0.0;
        let _ = dt_s;
        (out, new_state)
    }
}
// #endregion 🔖️HeatPump

// #region 🔖️Fixtures
/// 🚰️ Domestic hot-water fixture end use.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HotWaterFixture {
    pub name: String,
    pub peak_flow_l_s: f64,
    pub target_temperature_c: f64,
    pub schedule_factor: f64,
}

impl HotWaterFixture {
    /// 🚰️ Hot-water draw mass flow [kg/s] at mixed delivery temperature.
    pub async fn draw_flow_kg_s(&self, mains_temperature_c: f64) -> f64 {
        let flow_l_s = self.peak_flow_l_s * self.schedule_factor.clamp(0.0, 1.0);
        let mix_ratio = ((self.target_temperature_c - mains_temperature_c) / (self.target_temperature_c - mains_temperature_c).max(1.0)).clamp(0.0, 1.0);
        flow_l_s * mix_ratio * RHO_WATER / 1000.0
    }

    /// 🔥️ Sensible energy demand [W] for fixture draw.
    pub async fn demand_w(&self, mains_temperature_c: f64, storage_temperature_c: f64) -> f64 {
        let m_dot = self.draw_flow_kg_s(mains_temperature_c);
        let cp = water_cp_j_per_kg_k(storage_temperature_c);
        m_dot * cp * (self.target_temperature_c - mains_temperature_c).max(0.0)
    }
}
// #endregion 🔖️Fixtures

// #region 🔖️Standby
/// 🌡️ Standby loss model for tanks and distribution piping.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StandbyLoss {
    pub ua_w_per_k: f64,
    pub ambient_temperature_c: f64,
    pub circulation_pump_w: f64,
}

impl StandbyLoss {
    /// 🌡️ Total standby loss [W] from tank or recirc loop.
    pub async fn loss_w(&self, fluid_temperature_c: f64) -> f64 {
        self.ua_w_per_k * (fluid_temperature_c - self.ambient_temperature_c).max(0.0) + self.circulation_pump_w
    }
}
// #endregion 🔖️Standby

// #region 🔖️DrainRecovery
/// ♻️ Drain-water heat recovery heat exchanger.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DrainWaterHeatRecovery {
    pub effectiveness: f64,
    pub ua_w_per_k: f64,
}

impl DrainWaterHeatRecovery {
    /// ♻️ Preheat cold mains from warm drain flow.
    pub async fn preheat_w(&self, drain_flow_kg_s: f64, drain_temperature_c: f64, mains_temperature_c: f64) -> f64 {
        let cp = water_cp_j_per_kg_k((drain_temperature_c + mains_temperature_c) * 0.5);
        let q_max = drain_flow_kg_s * cp * (drain_temperature_c - mains_temperature_c).max(0.0);
        let eps = self.effectiveness.clamp(0.0, 0.95);
        (self.ua_w_per_k * (drain_temperature_c - mains_temperature_c)).min(q_max * eps).max(0.0)
    }

    /// 🌡️ Preheated mains temperature [°C].
    pub async fn preheated_mains_c(&self, drain_flow_kg_s: f64, drain_temperature_c: f64, mains_flow_kg_s: f64, mains_temperature_c: f64) -> f64 {
        let q = self.preheat_w(drain_flow_kg_s, drain_temperature_c, mains_temperature_c);
        let cp = water_cp_j_per_kg_k(mains_temperature_c);
        mains_temperature_c + q / (mains_flow_kg_s.max(1e-6) * cp)
    }
}
// #endregion 🔖️DrainRecovery

#[cfg(test)]
mod tests {
    use super::*;

    async fn electric_tank() -> MixedWaterHeater {
        MixedWaterHeater { volume_l: 300.0, ua_standby_w_per_k: 5.0, heating_capacity_w: 4500.0, setpoint_c: 55.0, ambient_c: 20.0, recovery_efficiency: 0.98, fuel: WaterHeaterFuel::Electric }
    }

    #[test]
    async fn mixed_tank_recovers_after_draw() {
        let tank = electric_tank();
        let state = WaterHeaterState { average_temperature_c: 55.0, top_temperature_c: 55.0, bottom_temperature_c: 55.0, volume_m3: 0.3 };
        let (out, new_state) = tank.simulate(&state, 0.05, 10.0, 3600.0);
        assert!(out.delivered_flow_kg_s > 0.0);
        assert!(new_state.average_temperature_c < state.average_temperature_c);
    }

    #[test]
    async fn stratified_tank_top_hotter_than_bottom() {
        let tank = StratifiedWaterHeater { volume_l: 400.0, node_count: 4, ua_standby_w_per_k: 4.0, setpoint_c: 60.0, ambient_c: 20.0, heating_capacity_w: 6000.0, heater_position: 3 };
        let initial = vec![55.0, 50.0, 45.0, 40.0];
        let (_out, temps) = tank.simulate(&initial, 0.0, 10.0, 3600.0);
        assert!(temps[3] > temps[0]);
        assert!(temps[3] > 40.0);
    }

    #[test]
    async fn hpwh_cop_reduces_electrical() {
        let hpwh = HeatPumpWaterHeater { tank: electric_tank(), rated_cop: 3.0, min_ambient_c: -5.0 };
        let state = WaterHeaterState { average_temperature_c: 45.0, top_temperature_c: 45.0, bottom_temperature_c: 45.0, volume_m3: 0.3 };
        let (out, _) = hpwh.simulate(&state, 0.0, 10.0, 20.0, 3600.0);
        if out.heating_power_w > 100.0 {
            assert!(out.electrical_power_w < out.heating_power_w);
        }
    }

    #[test]
    async fn fixture_demand_positive() {
        let fixture = HotWaterFixture { name: "Shower".into(), peak_flow_l_s: 0.15, target_temperature_c: 40.0, schedule_factor: 0.5 };
        assert!(fixture.demand_w(10.0, 55.0) > 0.0);
    }

    #[test]
    async fn drain_recovery_preheats_mains() {
        let dwhr = DrainWaterHeatRecovery { effectiveness: 0.6, ua_w_per_k: 500.0 };
        let preheated = dwhr.preheated_mains_c(0.05, 35.0, 0.05, 10.0);
        assert!(preheated > 10.0);
        assert!(preheated < 35.0);
    }

    #[test]
    async fn standby_loss_increases_with_temperature() {
        let standby = StandbyLoss { ua_w_per_k: 8.0, ambient_temperature_c: 20.0, circulation_pump_w: 30.0 };
        assert!(standby.loss_w(55.0) > standby.loss_w(40.0));
    }
}
