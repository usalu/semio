//! 💧️ Water systems: fixtures, tanks, rainwater, condensate, irrigation, cooling tower makeup.

use crate::props::water_density;
use crate::units::RHO_WATER;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

// #region 🔖️Fixture
/// 🚰️ Generic water fixture end use.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct WaterFixture {
    pub name: String,
    pub fixture_type: FixtureType,
    pub peak_flow_l_s: f64,
    pub schedule_factor: f64,
    pub hot_water_fraction: f64,
}

/// 🚰️ Standard fixture categories.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub enum FixtureType {
    Lavatory,
    Shower,
    KitchenSink,
    Toilet,
    Urinal,
    HoseBib,
    CoolingTower,
}

impl WaterFixture {
    /// 💧️ Volumetric flow [m³/s].
    pub fn flow_m3_s(&self) -> f64 {
        self.peak_flow_l_s * self.schedule_factor.clamp(0.0, 1.0) / 1000.0
    }

    /// 💧️ Mass flow [kg/s].
    pub fn mass_flow_kg_s(&self) -> f64 {
        self.flow_m3_s() * RHO_WATER
    }

    /// 🔥️ Hot-water branch flow [kg/s].
    pub fn hot_flow_kg_s(&self) -> f64 {
        self.mass_flow_kg_s() * self.hot_water_fraction.clamp(0.0, 1.0)
    }
}
// #endregion 🔖️Fixture

// #region 🔖️Tank
/// 🛢️ Potable or process water storage tank.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct WaterTank {
    pub volume_m3: f64,
    pub level_m: f64,
    pub max_level_m: f64,
    pub min_level_m: f64,
    pub inlet_temperature_c: f64,
}

impl WaterTank {
    /// 🛢️ Update tank level from inflow/outflow over dt.
    pub fn simulate(&self, inflow_m3_s: f64, outflow_m3_s: f64, dt_s: f64, area_m2: f64) -> (f64, f64) {
        let net_m3 = (inflow_m3_s - outflow_m3_s) * dt_s;
        let delta_level = net_m3 / area_m2.max(0.01);
        let new_level = (self.level_m + delta_level).clamp(self.min_level_m, self.max_level_m);
        let volume = new_level * area_m2;
        (new_level, volume)
    }

    /// 💧️ Available draw before hitting minimum level.
    pub fn available_volume_m3(&self, area_m2: f64) -> f64 {
        ((self.level_m - self.min_level_m).max(0.0)) * area_m2
    }
}
// #endregion 🔖️Tank

// #region 🔖️Rainwater
/// 🌧️ Rainwater harvesting from roof catchment.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct RainwaterSystem {
    pub catchment_area_m2: f64,
    pub runoff_coefficient: f64,
    pub tank: WaterTank,
    pub first_flush_l: f64,
    pub filter_efficiency: f64,
}

impl RainwaterSystem {
    /// 🌧️ Harvested volume [m³] from rainfall depth [mm] over timestep.
    pub fn harvest_m3(&self, rainfall_mm: f64, _dt_s: f64) -> f64 {
        let gross_m3 = self.catchment_area_m2 * rainfall_mm / 1000.0 * self.runoff_coefficient;
        let first_flush_m3 = if rainfall_mm > 0.0 { (self.first_flush_l / 1000.0).min(gross_m3) } else { 0.0 };
        (gross_m3 - first_flush_m3).max(0.0) * self.filter_efficiency
    }

    /// 🌧️ Simulate tank level with rainfall and demand.
    pub fn simulate(&self, rainfall_mm: f64, demand_m3_s: f64, dt_s: f64, tank_area_m2: f64) -> (f64, f64) {
        let harvest = self.harvest_m3(rainfall_mm, dt_s);
        let inflow = harvest / dt_s.max(1.0);
        let outflow = demand_m3_s.min(self.tank.available_volume_m3(tank_area_m2) / dt_s.max(1.0));
        let (level, volume) = self.tank.simulate(inflow, outflow, dt_s, tank_area_m2);
        (level, volume)
    }
}
// #endregion 🔖️Rainwater

// #region 🔖️Condensate
/// 💧️ HVAC condensate recovery from cooling coils.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct CondensateRecovery {
    pub collection_efficiency: f64,
    pub storage_tank: WaterTank,
}

impl CondensateRecovery {
    /// 💧️ Condensate mass flow [kg/s] from dehumidification rate.
    pub fn condensate_kg_s(&self, dehumidification_kg_s: f64) -> f64 {
        dehumidification_kg_s * self.collection_efficiency.clamp(0.0, 1.0)
    }

    /// 💧️ Accumulated condensate volume [m³] over timestep.
    pub fn accumulate_m3(&self, dehumidification_kg_s: f64, dt_s: f64) -> f64 {
        let kg = self.condensate_kg_s(dehumidification_kg_s) * dt_s;
        kg / water_density(20.0)
    }
}
// #endregion 🔖️Condensate

// #region 🔖️Irrigation
/// 🌱️ Landscape irrigation demand.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct IrrigationSystem {
    pub landscaped_area_m2: f64,
    pub crop_coefficient: f64,
    pub irrigation_efficiency: f64,
    pub precipitation_mm_per_day: f64,
}

impl IrrigationSystem {
    /// 🌱️ Irrigation water demand [m³/s] from reference evapotranspiration [mm/day].
    pub fn demand_m3_s(&self, et0_mm_per_day: f64, schedule_factor: f64) -> f64 {
        let et_c = et0_mm_per_day * self.crop_coefficient;
        let net_mm = (et_c - self.precipitation_mm_per_day).max(0.0);
        let gross_mm = net_mm / self.irrigation_efficiency.max(0.1);
        let m3_per_day = self.landscaped_area_m2 * gross_mm / 1000.0;
        m3_per_day / 86_400.0 * schedule_factor.clamp(0.0, 1.0)
    }
}
// #endregion 🔖️Irrigation

// #region 🔖️CoolingTowerMakeup
/// 🌊️ Cooling tower evaporation, drift, and blowdown makeup water.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct CoolingTowerMakeup {
    pub cycles_of_concentration: f64,
    pub drift_fraction: f64,
    pub basin_volume_m3: f64,
}

impl CoolingTowerMakeup {
    /// 🌊️ Evaporation rate [kg/s] from condenser heat rejection.
    pub fn evaporation_kg_s(&self, heat_rejection_w: f64, delta_h_vaporization_j_kg: f64) -> f64 {
        if heat_rejection_w <= 0.0 {
            return 0.0;
        }
        heat_rejection_w / delta_h_vaporization_j_kg.max(1.0)
    }

    /// 🌊️ Total makeup water [kg/s] including blowdown and drift.
    pub fn makeup_kg_s(&self, heat_rejection_w: f64) -> f64 {
        let h_fg = 2_400_000.0;
        let evap = self.evaporation_kg_s(heat_rejection_w, h_fg);
        let coc = self.cycles_of_concentration.max(1.5);
        let blowdown = evap / (coc - 1.0);
        let drift = evap * self.drift_fraction;
        evap + blowdown + drift
    }

    /// 🌊️ Annual makeup [m³] from average heat rejection.
    pub fn annual_makeup_m3(&self, average_rejection_w: f64) -> f64 {
        self.makeup_kg_s(average_rejection_w) * 86_400.0 * 365.0 / RHO_WATER
    }
}
// #endregion 🔖️CoolingTowerMakeup

// #region 🔖️Balance
/// 💧️ Building water balance for one timestep.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct WaterBalance {
    pub fixture_demand_m3_s: f64,
    pub irrigation_demand_m3_s: f64,
    pub cooling_makeup_m3_s: f64,
    pub rainwater_supply_m3_s: f64,
    pub condensate_supply_m3_s: f64,
    pub mains_import_m3_s: f64,
}

/// 💧️ Net mains water demand [m³/s].
pub fn water_balance(fixtures: &[WaterFixture], irrigation_m3_s: f64, cooling_makeup_kg_s: f64, rainwater_m3_s: f64, condensate_kg_s: f64) -> WaterBalance {
    let fixture_demand: f64 = fixtures.iter().map(|f| f.flow_m3_s()).sum();
    let condensate_m3_s = condensate_kg_s / RHO_WATER;
    let cooling_m3_s = cooling_makeup_kg_s / RHO_WATER;
    let supply = rainwater_m3_s + condensate_m3_s;
    let demand = fixture_demand + irrigation_m3_s + cooling_m3_s;
    WaterBalance {
        fixture_demand_m3_s: fixture_demand,
        irrigation_demand_m3_s: irrigation_m3_s,
        cooling_makeup_m3_s: cooling_m3_s,
        rainwater_supply_m3_s: rainwater_m3_s,
        condensate_supply_m3_s: condensate_m3_s,
        mains_import_m3_s: (demand - supply).max(0.0),
    }
}
// #endregion 🔖️Balance

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
