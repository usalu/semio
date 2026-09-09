use super::*;
use crate::standards::energyplus::subsets::any::schema::snapshot::{EpwRecord, STDIO_EPW_DOCUMENT_SCHEMA};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn record(dry_bulb_temp: &str) -> EpwRecord {
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
