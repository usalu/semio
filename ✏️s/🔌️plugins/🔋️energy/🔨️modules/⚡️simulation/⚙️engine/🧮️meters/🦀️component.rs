//! ⚡️ Energy and resource meters with end-use categories.

use crate::model::FixedTable;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

// #region 🔖️Fuel
/// ⛽️ Fuel/resource type for meters.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct MeterTable {
    pub(crate) meters: FixedTable<String, Meter>,
}

impl MeterTable {
    #[cfg(test)]
    pub(crate) fn get_or_create(&mut self, name: &str, fuel: FuelType, end_use: EndUse) -> &mut Meter {
        if self.meters.capacity() == 0 {
            self.meters.admit(64).expect("test meter backing");
        }
        if let Some(index) = self.meters.test_index_of(|candidate| candidate == name) {
            return self.meters.get_index_mut(index).expect("test fixed meter slot");
        }
        let name = name.to_string();
        self.meters.insert_stable(name.clone(), Meter { name, fuel, end_use, energy_j: 0.0, peak_demand_w: 0.0, peak_demand_hour: 0.0 }).expect("test fixed meter slot");
        self.meters.last_mut().expect("test fixed meter slot").1
    }

    #[cfg(test)]
    pub(crate) fn facility_total_kwh(&self, fuel: FuelType) -> f64 {
        self.meters.values().filter(|m| m.fuel == fuel).map(|m| m.energy_kwh()).sum()
    }

    #[cfg(test)]
    pub(crate) fn end_use_breakdown(&self) -> Vec<(EndUse, f64)> {
        let mut values = Vec::new();
        for m in self.meters.values() {
            if let Some((_, value)) = values.iter_mut().find(|(end_use, _)| *end_use == m.end_use) {
                *value += m.energy_kwh();
            } else {
                values.push((m.end_use, m.energy_kwh()));
            }
        }
        values
    }
}
// #endregion 🔖️Meter

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meter_accumulates_energy() {
        let mut m = Meter { name: "test".into(), fuel: FuelType::Electricity, end_use: EndUse::Heating, energy_j: 0.0, peak_demand_w: 0.0, peak_demand_hour: 0.0 };
        m.accumulate(1000.0, 3600.0, 1.0);
        assert!((m.energy_kwh() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn meter_tracks_peak_demand_hour() {
        let mut m = Meter { name: "test".into(), fuel: FuelType::Electricity, end_use: EndUse::Cooling, energy_j: 0.0, peak_demand_w: 0.0, peak_demand_hour: 0.0 };
        m.accumulate(500.0, 3600.0, 1.0);
        m.accumulate(1500.0, 3600.0, 2.0);
        m.accumulate(200.0, 3600.0, 3.0);
        assert!((m.peak_demand_w - 1500.0).abs() < 1e-9);
        assert!((m.peak_demand_hour - 2.0).abs() < 1e-9);
    }

    #[test]
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

    #[test]
    fn end_use_breakdown_aggregates_by_category() {
        let mut store = MeterTable::default();
        store.get_or_create("Zone1 Heating", FuelType::Electricity, EndUse::Heating).accumulate(1000.0, 3600.0, 0.0);
        store.get_or_create("Zone2 Heating", FuelType::Electricity, EndUse::Heating).accumulate(1000.0, 3600.0, 0.0);
        store.get_or_create("Fans", FuelType::Electricity, EndUse::Fans).accumulate(500.0, 3600.0, 0.0);
        let breakdown = store.end_use_breakdown();
        let heating = breakdown.iter().find(|(end_use, _)| *end_use == EndUse::Heating).map(|(_, value)| *value).unwrap();
        let fans = breakdown.iter().find(|(end_use, _)| *end_use == EndUse::Fans).map(|(_, value)| *value).unwrap();
        assert!((heating - 2.0 * 1000.0 * 3600.0 / 3_600_000.0).abs() < 1e-6);
        assert!((fans - 500.0 * 3600.0 / 3_600_000.0).abs() < 1e-6);
    }
}
