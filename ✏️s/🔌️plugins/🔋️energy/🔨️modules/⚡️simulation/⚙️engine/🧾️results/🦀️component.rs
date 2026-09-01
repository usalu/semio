//! 📋️ Canonical simulation results and summary tables.

use crate::error::Diagnostics;
use crate::meters::MeterTable;
use crate::metrics::{EnvironmentalMetrics, ResilienceMetrics};
use crate::output::TimeSeriesTable;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

// #region 🔖️Summary
/// 📋️ Annual/monthly summary table row.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct SummaryRow {
    pub key: String,
    pub value: f64,
    pub unit: String,
}

/// 📋️ Summary tables (energy use, loads, comfort).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct SummaryTables {
    pub annual_energy: Vec<SummaryRow>,
    pub monthly_energy: Vec<(u8, Vec<SummaryRow>)>,
    pub peak_loads: Vec<SummaryRow>,
    pub comfort: Vec<SummaryRow>,
}

impl SummaryTables {
    pub fn add_annual(&mut self, key: impl Into<String>, value: f64, unit: impl Into<String>) {
        self.annual_energy.push(SummaryRow { key: key.into(), value, unit: unit.into() });
    }
}
// #endregion 🔖️Summary

// #region 🔖️Sizing
/// 📐️ Component sizing result.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct SizingResult {
    pub component: String,
    pub design_load_w: f64,
    pub design_flow_m3_s: f64,
    pub autosized: bool,
}

/// 📐️ Sizing tables from design-day calculations.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct SizingTables {
    pub zone_loads: Vec<SizingResult>,
    pub equipment: Vec<SizingResult>,
}
// #endregion 🔖️Sizing

// #region 🔖️Results
/// 📋️ Complete simulation results (canonical structured format).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct Results {
    pub time_series: TimeSeriesTable,
    pub meters: MeterTable,
    pub summaries: SummaryTables,
    pub sizing: SizingTables,
    pub environmental: EnvironmentalMetrics,
    pub resilience: ResilienceMetrics,
    pub diagnostics: Diagnostics,
    pub run_metadata: RunMetadata,
}

/// 🏷️ Run metadata.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
pub struct RunMetadata {
    pub model_name: String,
    pub model_version: String,
    pub weather_location: String,
    pub timesteps: u32,
    pub warmup_days: u32,
    pub elapsed_ms: u64,
}
// #endregion 🔖️Results

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summary_tables_accumulate() {
        let mut s = SummaryTables::default();
        s.add_annual("Electricity", 1000.0, "kWh");
        assert_eq!(s.annual_energy.len(), 1);
    }
}
