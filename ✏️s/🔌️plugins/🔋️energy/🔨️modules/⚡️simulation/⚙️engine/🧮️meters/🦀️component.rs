//! ⚡️ Energy and resource meters with end-use categories.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// #region 🔖️Fuel
/// ⛽️ Fuel/resource type for meters.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FuelType {
    Electricity,
    NaturalGas,
    Propane,
    FuelOil,
    DistrictHeating,
    DistrictCooling,
    Steam,
    Water,
    OnSiteGeneration,
}

/// 📊️ End-use category.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EndUse {
    Heating,
    Cooling,
    InteriorLighting,
    ExteriorLighting,
    InteriorEquipment,
    ExteriorEquipment,
    Fans,
    Pumps,
    HeatRejection,
    Humidification,
    Dehumidification,
    WaterSystems,
    Refrigeration,
    Generators,
    Custom(u32),
}
// #endregion 🔖️Fuel

// #region 🔖️Meter
/// ⚡️ Single meter reading accumulator.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Meter {
    pub name: String,
    pub fuel: FuelType,
    pub end_use: EndUse,
    pub energy_j: f64,
    pub peak_demand_w: f64,
    pub peak_demand_hour: f64,
}

impl Meter {
    pub fn accumulate(&mut self, power_w: f64, dt_s: f64, hour: f64) {
        self.energy_j += power_w * dt_s;
        if power_w > self.peak_demand_w {
            self.peak_demand_w = power_w;
            self.peak_demand_hour = hour;
        }
    }

    pub fn energy_kwh(&self) -> f64 {
        self.energy_j / 3_600_000.0
    }
}

/// 📦️ All meters in a simulation run.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MeterTable {
    pub meters: HashMap<String, Meter>,
}

impl MeterTable {
    pub fn get_or_create(&mut self, name: &str, fuel: FuelType, end_use: EndUse) -> &mut Meter {
        self.meters.entry(name.to_string()).or_insert_with(|| Meter { name: name.to_string(), fuel, end_use, energy_j: 0.0, peak_demand_w: 0.0, peak_demand_hour: 0.0 })
    }

    pub fn facility_total_kwh(&self, fuel: FuelType) -> f64 {
        self.meters.values().filter(|m| m.fuel == fuel).map(|m| m.energy_kwh()).sum()
    }

    pub fn end_use_breakdown(&self) -> HashMap<EndUse, f64> {
        let mut map = HashMap::new();
        for m in self.meters.values() {
            *map.entry(m.end_use).or_insert(0.0) += m.energy_kwh();
        }
        map
    }
}
// #endregion 🔖️Meter

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    fn meter_accumulates_energy() {
        let mut m = Meter { name: "test".into(), fuel: FuelType::Electricity, end_use: EndUse::Heating, energy_j: 0.0, peak_demand_w: 0.0, peak_demand_hour: 0.0 };
        m.accumulate(1000.0, 3600.0, 1.0);
        assert!((m.energy_kwh() - 1.0).abs() < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    fn meter_tracks_peak_demand_hour() {
        let mut m = Meter { name: "test".into(), fuel: FuelType::Electricity, end_use: EndUse::Cooling, energy_j: 0.0, peak_demand_w: 0.0, peak_demand_hour: 0.0 };
        m.accumulate(500.0, 3600.0, 1.0);
        m.accumulate(1500.0, 3600.0, 2.0);
        m.accumulate(200.0, 3600.0, 3.0);
        assert!((m.peak_demand_w - 1500.0).abs() < 1e-9);
        assert!((m.peak_demand_hour - 2.0).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    fn store_get_or_create_is_idempotent_and_totals_by_fuel() {
        let mut store = MeterTable::default();
        store.get_or_create("Zone1 Heating", FuelType::Electricity, EndUse::Heating).accumulate(1000.0, 3600.0, 0.0);
        store.get_or_create("Zone1 Heating", FuelType::Electricity, EndUse::Heating).accumulate(1000.0, 3600.0, 1.0);
        store.get_or_create("Boiler Gas", FuelType::NaturalGas, EndUse::Heating).accumulate(2000.0, 3600.0, 0.0);
        assert_eq!(store.meters.len(), 2);
        assert!((store.facility_total_kwh(FuelType::Electricity) - 2.0).abs() < 1e-6);
        assert!((store.facility_total_kwh(FuelType::NaturalGas) - 2000.0 * 3600.0 / 3_600_000.0).abs() < 1e-6);
        assert_eq!(store.facility_total_kwh(FuelType::Propane), 0.0);
    }

    #[semio_framework_async_macros::async_test]
    fn end_use_breakdown_aggregates_by_category() {
        let mut store = MeterTable::default();
        store.get_or_create("Zone1 Heating", FuelType::Electricity, EndUse::Heating).accumulate(1000.0, 3600.0, 0.0);
        store.get_or_create("Zone2 Heating", FuelType::Electricity, EndUse::Heating).accumulate(1000.0, 3600.0, 0.0);
        store.get_or_create("Fans", FuelType::Electricity, EndUse::Fans).accumulate(500.0, 3600.0, 0.0);
        let breakdown = store.end_use_breakdown();
        assert!((breakdown[&EndUse::Heating] - 2.0 * 1000.0 * 3600.0 / 3_600_000.0).abs() < 1e-6);
        assert!((breakdown[&EndUse::Fans] - 500.0 * 3600.0 / 3_600_000.0).abs() < 1e-6);
    }
}
