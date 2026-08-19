//! 🌿️ Environmental and resilience metrics.

use serde::{Deserialize, Serialize};

// #region 🔖️Environmental
/// 🌿️ Source energy conversion factors by fuel [J/J delivered].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceEnergyFactors {
    pub electricity: f64,
    pub natural_gas: f64,
    pub district_heating: f64,
    pub district_cooling: f64,
}

impl Default for SourceEnergyFactors {
    async fn default() -> Self {
        Self { electricity: 3.0, natural_gas: 1.05, district_heating: 1.2, district_cooling: 1.1 }
    }
}

/// 🌿️ Greenhouse gas emission factors [kg CO2e per kWh].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmissionFactors {
    pub electricity_kg_per_kwh: f64,
    pub natural_gas_kg_per_kwh: f64,
}

impl Default for EmissionFactors {
    async fn default() -> Self {
        Self { electricity_kg_per_kwh: 0.4, natural_gas_kg_per_kwh: 0.2 }
    }
}

/// 🌿️ Environmental metrics summary.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentalMetrics {
    pub site_energy_kwh: f64,
    pub source_energy_kwh: f64,
    pub co2_kg: f64,
}
// #endregion 🔖️Environmental

// #region 🔖️Resilience
/// 🛡️ Resilience exposure metrics.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ResilienceMetrics {
    pub hours_above_heat_index_32c: u32,
    pub hours_below_10c: u32,
    pub unmet_heating_hours: u32,
    pub unmet_cooling_hours: u32,
    pub passive_survivability_hours: u32,
}

/// 🛡️ Compute resilience metrics from zone temperature time series.
pub async fn compute_resilience(zone_temps_c: &[f64], heating_setpoint_c: f64, cooling_setpoint_c: f64, hvac_available: bool) -> ResilienceMetrics {
    let mut m = ResilienceMetrics::default();
    for &t in zone_temps_c {
        if t > 32.0 {
            m.hours_above_heat_index_32c += 1;
        }
        if t < 10.0 {
            m.hours_below_10c += 1;
        }
        if !hvac_available {
            m.passive_survivability_hours += 1;
        }
        if t < heating_setpoint_c - 1.0 {
            m.unmet_heating_hours += 1;
        }
        if t > cooling_setpoint_c + 1.0 {
            m.unmet_cooling_hours += 1;
        }
    }
    m
}
// #endregion 🔖️Resilience

// #region 🔖️Compute
/// 🌿️ Compute environmental metrics from meter totals.
pub async fn compute_environmental(electricity_kwh: f64, gas_kwh: f64, factors: &SourceEnergyFactors, emissions: &EmissionFactors) -> EnvironmentalMetrics {
    let site = electricity_kwh + gas_kwh;
    let source = electricity_kwh * factors.electricity + gas_kwh * factors.natural_gas;
    let co2 = electricity_kwh * emissions.electricity_kg_per_kwh + gas_kwh * emissions.natural_gas_kg_per_kwh;
    EnvironmentalMetrics { site_energy_kwh: site, source_energy_kwh: source, co2_kg: co2 }
}
// #endregion 🔖️Compute

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn resilience_counts_extremes() {
        let temps = vec![35.0, 5.0, 22.0];
        let r = compute_resilience(&temps, 20.0, 26.0, true);
        assert_eq!(r.hours_above_heat_index_32c, 1);
        assert_eq!(r.hours_below_10c, 1);
    }
}
