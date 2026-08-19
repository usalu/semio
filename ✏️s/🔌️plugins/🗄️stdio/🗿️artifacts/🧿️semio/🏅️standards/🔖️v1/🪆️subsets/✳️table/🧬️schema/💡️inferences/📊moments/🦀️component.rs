//! 📊 `moments` — real per-column descriptive moments, computed as a genuine
//! `InferredField<SemioTableSnapshot>` (not a bare pass-through): one step per NUMERIC (`Int`/
//! `Float`) column, keyed by column NAME (the native key, per this subset's own `SemioTableColumn`
//! doc comment), NO parents — a column's moments depend only on its OWN cell values, never on any
//! other column's, mirroring `✳️mesh`'s `📦aabb` pilot's per-primitive chain shape.
//!
//! Wraps `📊️statistics-internals::{mean, variance, std_dev}` — moved verbatim from
//! `🧰️framework/🔨️modules/🧮️math/📊️statistics` in ticket 26/08/12/
//! DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave M3c. This is the proof that the
//! relocated compute internals are genuinely wired as an inference, not merely relocated library
//! code sitting under a directory named `💡️inferences/` (the m3a lesson: a directory name is not a
//! mechanism).
//!
//! NOT YET wired into the parent `SemioTableInference` aggregate struct or its hand-rolled
//! binary/text/json/proto/graphql codecs — that round-trip surface is separate, higher-risk work
//! (five codec formats to keep byte-for-byte consistent) out of scope for this pass. This field is
//! real and independently tested (see below) via `store::infer_field` directly; wiring it into the
//! aggregate is an honest, flagged remainder, not a silently dropped step.

use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::{SemioTableCellKind, SemioTableSnapshot};
use crate::artifacts::semio::standards::v1::subsets::table::schema::statistics_internals;
use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValue;
use serde::{Deserialize, Serialize};

//#region 🔖️Value
/// 📊️ One numeric column's descriptive moments. `SemioColumnMoments::default()` (all-zero) is the
/// honest "no numeric data" value for a column with zero parseable cells — same convention
/// `✳️mesh`'s `SemioAabb::default()` uses for "no geometry".
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioColumnMoments {
    pub count: u32,
    pub mean: f64,
    pub variance: f64,
    pub std_dev: f64,
}
//#endregion 🔖️Value

//#region 🔖️Lookup
/// 🔎 Positionally aligned column-name → numeric-cell-values extraction. `Int`/`Float` cells parse
/// their `lexeme`; any other cell kind (including a stray `Str`/`Bool`/`Null` in a nominally
/// numeric column — this format enforces no runtime kind check, per `📐shape`'s own doc comment) is
/// skipped rather than treated as zero, so a mixed column's moments stay honest about `count`.
async fn numeric_cell_value(cell: &SemioValue) -> Option<f64> {
    match cell {
        SemioValue::Int { lexeme } => lexeme.parse::<f64>().ok(),
        SemioValue::Float { lexeme } => lexeme.parse::<f64>().ok(),
        _ => None,
    }
}

async fn column_values(snapshot: &SemioTableSnapshot, column_name: &str) -> Vec<f64> {
    let Some(idx) = snapshot.columns.iter().position(|c| c.name == column_name) else {
        return Vec::new();
    };
    snapshot.rows.iter().filter_map(|row| row.cells.get(idx)).filter_map(numeric_cell_value).collect()
}
//#endregion 🔖️Lookup

//#region 🔖️DependencyHashChain
pub struct ColumnMoments;

impl store::InferredField<SemioTableSnapshot> for ColumnMoments {
    type Key = String;
    type Value = SemioColumnMoments;
    const FIELD_ID: &'static str = "s.stdio.semio.table.inference.moments";
    const SCHEMA_VERSION: u32 = 1;

    async fn reads() -> &'static [&'static str] {
        &["columns", "rows"]
    }

    async fn plan(snapshot: &SemioTableSnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        snapshot.columns.iter().filter(|c| matches!(c.kind, SemioTableCellKind::Int | SemioTableCellKind::Float)).map(|c| store::InferenceStep { key: c.name.clone(), parents: Vec::new() }).collect()
    }

    /// 🔑 Canonical dependency-input bytes — EXACTLY this column's own numeric cell values, nothing
    /// else (not other columns, not `kind`, which `plan` already gates on) — an unrelated column's
    /// edit must still hit the cache, proven by the incrementality-law test below.
    async fn dep_input(snapshot: &SemioTableSnapshot, key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        serde_json::to_vec(&column_values(snapshot, key)).unwrap_or_default()
    }

    async fn compute(snapshot: &SemioTableSnapshot, key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        let values = column_values(snapshot, key);
        let count = values.len() as u32;
        let mean = statistics_internals::mean(&values).unwrap_or(0.0);
        let variance = statistics_internals::variance(&values).unwrap_or(0.0);
        let std_dev = statistics_internals::std_dev(&values).unwrap_or(0.0);
        SemioColumnMoments { count, mean, variance, std_dev }
    }
}
//#endregion 🔖️DependencyHashChain

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::{SemioTableColumn, SemioTableRow, STDIO_SEMIOTABLE_DOCUMENT_SCHEMA};
    use store::{InferenceCache, InferenceCacheConfig};

    async fn two_numeric_column_snapshot() -> SemioTableSnapshot {
        SemioTableSnapshot {
            schema: STDIO_SEMIOTABLE_DOCUMENT_SCHEMA.into(),
            columns: vec![SemioTableColumn { name: "score".into(), kind: SemioTableCellKind::Float }, SemioTableColumn { name: "count".into(), kind: SemioTableCellKind::Int }, SemioTableColumn { name: "label".into(), kind: SemioTableCellKind::Str }],
            rows: vec![
                SemioTableRow { cells: vec![SemioValue::Float { lexeme: "1.0".into() }, SemioValue::Int { lexeme: "10".into() }, SemioValue::Str { value: "a".into() }] },
                SemioTableRow { cells: vec![SemioValue::Float { lexeme: "2.0".into() }, SemioValue::Int { lexeme: "20".into() }, SemioValue::Str { value: "b".into() }] },
                SemioTableRow { cells: vec![SemioValue::Float { lexeme: "3.0".into() }, SemioValue::Int { lexeme: "30".into() }, SemioValue::Str { value: "c".into() }] },
            ],
        }
    }

    //#region 🧪️Honesty
    #[semio_framework_async_macros::async_test]
    async fn moments_of_a_populated_numeric_column_are_the_real_descriptive_stats() {
        let values = store::infer_field::<SemioTableSnapshot, ColumnMoments>(&two_numeric_column_snapshot(), None);
        let m = values.get("score").expect("score moments present");
        assert_eq!(m.count, 3);
        assert!((m.mean - 2.0).abs() < 1e-9);
        assert!((m.variance - 1.0).abs() < 1e-9);
        assert!((m.std_dev - 1.0).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn a_declared_str_column_is_absent_from_the_plan_not_a_faked_zero() {
        let values = store::infer_field::<SemioTableSnapshot, ColumnMoments>(&two_numeric_column_snapshot(), None);
        assert!(values.get("label").is_none(), "non-numeric columns must not appear in the plan at all");
    }

    #[semio_framework_async_macros::async_test]
    async fn moments_of_an_all_empty_snapshot_yields_an_empty_plan() {
        let values = store::infer_field::<SemioTableSnapshot, ColumnMoments>(&SemioTableSnapshot::default(), None);
        assert!(values.is_empty());
    }
    //#endregion 🧪️Honesty

    //#region 🧪️CacheTransparencyLaw
    #[semio_framework_async_macros::async_test]
    async fn disabled_cache_matches_pure_recompute() {
        let snapshot = two_numeric_column_snapshot();
        let pure = store::infer_field::<SemioTableSnapshot, ColumnMoments>(&snapshot, None);
        let mut disabled = InferenceCache::new(InferenceCacheConfig { enabled: false, ..Default::default() });
        let via_disabled = store::infer_field::<SemioTableSnapshot, ColumnMoments>(&snapshot, Some(&mut disabled));
        assert_eq!(pure, via_disabled);
    }
    //#endregion 🧪️CacheTransparencyLaw

    //#region 🧪️IncrementalityLaw
    #[semio_framework_async_macros::async_test]
    async fn identical_snapshot_recompute_is_a_cache_hit() {
        let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() });
        let base = two_numeric_column_snapshot();
        let _ = store::infer_field::<SemioTableSnapshot, ColumnMoments>(&base, Some(&mut cache));
        let before = cache.stats();
        let _ = store::infer_field::<SemioTableSnapshot, ColumnMoments>(&base, Some(&mut cache));
        let after = cache.stats();
        assert_eq!(after.misses, before.misses, "an unchanged snapshot must produce zero new misses");
        assert_eq!(after.hits - before.hits, 2, "both numeric columns must be cache hits");
    }

    #[semio_framework_async_macros::async_test]
    async fn changing_one_columns_cells_misses_only_that_columns_cache_entry() {
        let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() });
        let base = two_numeric_column_snapshot();
        let _ = store::infer_field::<SemioTableSnapshot, ColumnMoments>(&base, Some(&mut cache));

        let mut changed = base.clone();
        changed.rows[0].cells[0] = SemioValue::Float { lexeme: "99.0".into() };
        let before = cache.stats();
        let values = store::infer_field::<SemioTableSnapshot, ColumnMoments>(&changed, Some(&mut cache));
        let after = cache.stats();

        assert_eq!(after.misses - before.misses, 1, "only score's own entry may miss when its own cells change");
        assert_eq!(values.get("count").map(|m| m.count), Some(3), "count column's moments must be untouched");
    }

    #[semio_framework_async_macros::async_test]
    async fn changing_an_unrelated_column_does_not_miss() {
        let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() });
        let base = two_numeric_column_snapshot();
        let _ = store::infer_field::<SemioTableSnapshot, ColumnMoments>(&base, Some(&mut cache));

        let mut changed = base.clone();
        changed.rows[0].cells[2] = SemioValue::Str { value: "z".into() };
        let before = cache.stats();
        let _ = store::infer_field::<SemioTableSnapshot, ColumnMoments>(&changed, Some(&mut cache));
        let after = cache.stats();
        assert_eq!(after.misses, before.misses, "the label column has no bearing on score/count dep chains");
    }
    //#endregion 🧪️IncrementalityLaw
}
//#endregion 🧪️Tests
