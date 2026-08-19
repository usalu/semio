//! 🌡️ `climate` — one named inference: EPW is climate/weather data, not geometry, so the closest
//! honest derived statistic is a min/max/avg fold over the hourly dry-bulb air temperature column
//! (`EpwRecord::dry_bulb_temp`, a real EnergyPlus Weather field — see
//! https://bigladdersoftware.com/epx/docs/9-6/auxiliary-programs/energyplus-weather-file-epw-data-dictionary.html).
//! Every column is stored as `String` by design (see `📸️snapshot`'s own module doc comment), so
//! this fold attempts `dry_bulb_temp.trim().parse::<f64>()` per record and SKIPS (never
//! errors/panics) on parse failure — `recordCount` is every record seen (parsed or not),
//! `parsedTempCount` is how many parsed successfully, and `minDryBulbC`/`maxDryBulbC`/
//! `avgDryBulbC` fold only over the successfully-parsed values. Zero parsed values (including the
//! empty-`records` case) is the fold's identity: all three temperature fields default to `0.0`.

use crate::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::EpwSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ClimateSummary
/// 🌡️ Epw's hourly dry-bulb temperature min/max/avg.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpwClimateSummary {
    pub record_count: u32,
    pub parsed_temp_count: u32,
    pub min_dry_bulb_c: f64,
    pub max_dry_bulb_c: f64,
    pub avg_dry_bulb_c: f64,
}

/// 🩹 Hand-rolled: the fold's "no data" identity (zero parsed temperatures) must be written
/// explicitly rather than assumed to fall out of `#[derive(Default)]` — the divide-by-zero guard
/// logic lives in `compute_epw_climate_summary`, not in a derive.
impl Default for EpwClimateSummary {
    fn default() -> Self {
        Self { record_count: 0, parsed_temp_count: 0, min_dry_bulb_c: 0.0, max_dry_bulb_c: 0.0, avg_dry_bulb_c: 0.0 }
    }
}

/// 🌡️ Computes [`EpwClimateSummary`] via one pass over `records` — see module doc comment for the
/// exact parse/skip/fold rule.
pub async fn compute_epw_climate_summary(snapshot: &EpwSnapshot) -> EpwClimateSummary {
    let mut parsed_temp_count = 0u32;
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    let mut sum = 0.0f64;

    for record in &snapshot.records {
        if let Ok(value) = record.dry_bulb_temp.trim().parse::<f64>() {
            parsed_temp_count += 1;
            sum += value;
            min = min.min(value);
            max = max.max(value);
        }
    }

    let (min_dry_bulb_c, max_dry_bulb_c, avg_dry_bulb_c) = if parsed_temp_count == 0 { (0.0, 0.0, 0.0) } else { (min, max, sum / parsed_temp_count as f64) };

    EpwClimateSummary { record_count: snapshot.records.len() as u32, parsed_temp_count, min_dry_bulb_c, max_dry_bulb_c, avg_dry_bulb_c }
}
//#endregion 🔖️ClimateSummary

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::{EpwRecord, STDIO_EPW_DOCUMENT_SCHEMA};

    async fn record(dry_bulb_temp: &str) -> EpwRecord {
        EpwRecord { dry_bulb_temp: dry_bulb_temp.into(), ..Default::default() }
    }

    #[semio_framework_async_macros::async_test]
    async fn folds_parseable_temps_and_skips_malformed_ones() {
        let snapshot = EpwSnapshot { schema: STDIO_EPW_DOCUMENT_SCHEMA.into(), records: vec![record("10.0"), record("not-a-number"), record("30.0"), record("20.0")], ..Default::default() };
        let climate = compute_epw_climate_summary(&snapshot);
        assert_eq!(climate.record_count, 4);
        assert_eq!(climate.parsed_temp_count, 3);
        assert_eq!(climate.min_dry_bulb_c, 10.0);
        assert_eq!(climate.max_dry_bulb_c, 30.0);
        assert_eq!(climate.avg_dry_bulb_c, 20.0);
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snapshot = EpwSnapshot::default();
        assert_eq!(compute_epw_climate_summary(&snapshot), compute_epw_climate_summary(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(compute_epw_climate_summary(&EpwSnapshot::default()), EpwClimateSummary::default());
    }
}
//#endregion 🧪️Tests
