use super::*;
use crate::standards::v1::subsets::table::schema::snapshot::{SemioTableCellKind, SemioTableColumn, SemioTableRow, STDIO_SEMIOTABLE_DOCUMENT_SCHEMA};
use store::{InferenceCache, InferenceCacheConfig};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn two_column_snapshot() -> SemioTableSnapshot {
    SemioTableSnapshot {
        schema: STDIO_SEMIOTABLE_DOCUMENT_SCHEMA.into(),
        columns: vec![SemioTableColumn { name: "coin".into(), kind: SemioTableCellKind::Str }, SemioTableColumn { name: "always_a".into(), kind: SemioTableCellKind::Str }],
        rows: vec![
            SemioTableRow { cells: vec![SemioValue::Str { value: "heads".into() }, SemioValue::Str { value: "a".into() }] },
            SemioTableRow { cells: vec![SemioValue::Str { value: "tails".into() }, SemioValue::Str { value: "a".into() }] },
            SemioTableRow { cells: vec![SemioValue::Str { value: "heads".into() }, SemioValue::Str { value: "a".into() }] },
            SemioTableRow { cells: vec![SemioValue::Str { value: "tails".into() }, SemioValue::Str { value: "a".into() }] },
        ],
    }
}

//#region 🧪️Honesty
#[semio_framework_async_macros::async_test]
async fn a_fair_binary_column_has_one_bit_of_entropy() {
    let values = store::infer_field::<SemioTableSnapshot, ColumnEntropy>(&two_column_snapshot(), None);
    let e = values.get("coin").expect("coin entropy present");
    assert_eq!(e.count, 4);
    assert_eq!(e.distinct, 2);
    assert!((e.bits - 1.0).abs() < 1e-9, "fair coin must be exactly 1 bit, got {}", e.bits);
}

#[semio_framework_async_macros::async_test]
async fn a_constant_column_has_zero_entropy() {
    let values = store::infer_field::<SemioTableSnapshot, ColumnEntropy>(&two_column_snapshot(), None);
    let e = values.get("always_a").expect("always_a entropy present");
    assert_eq!(e.distinct, 1);
    assert!(e.bits.abs() < 1e-9, "single-symbol column must have zero entropy, got {}", e.bits);
}

#[semio_framework_async_macros::async_test]
async fn every_declared_column_appears_regardless_of_kind() {
    let values = store::infer_field::<SemioTableSnapshot, ColumnEntropy>(&two_column_snapshot(), None);
    assert_eq!(values.len(), 2, "entropy is defined over any symbol alphabet, unlike moments' numeric-only gate");
}

#[semio_framework_async_macros::async_test]
async fn an_all_empty_snapshot_yields_an_empty_plan() {
    let values = store::infer_field::<SemioTableSnapshot, ColumnEntropy>(&SemioTableSnapshot::default(), None);
    assert!(values.is_empty());
}
//#endregion 🧪️Honesty

//#region 🧪️CacheTransparencyLaw
#[semio_framework_async_macros::async_test]
async fn disabled_cache_matches_pure_recompute() {
    let snapshot = two_column_snapshot();
    let pure = store::infer_field::<SemioTableSnapshot, ColumnEntropy>(&snapshot, None);
    let mut disabled = InferenceCache::new(InferenceCacheConfig { enabled: false, ..Default::default() }).await;
    let via_disabled = store::infer_field::<SemioTableSnapshot, ColumnEntropy>(&snapshot, Some(&mut disabled));
    assert_eq!(pure, via_disabled);
}
//#endregion 🧪️CacheTransparencyLaw

//#region 🧪️IncrementalityLaw
#[semio_framework_async_macros::async_test]
async fn identical_snapshot_recompute_is_a_cache_hit() {
    let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() }).await;
    let base = two_column_snapshot();
    let _ = store::infer_field::<SemioTableSnapshot, ColumnEntropy>(&base, Some(&mut cache));
    let before = cache.stats().await;
    let _ = store::infer_field::<SemioTableSnapshot, ColumnEntropy>(&base, Some(&mut cache));
    let after = cache.stats().await;
    assert_eq!(after.misses, before.misses, "an unchanged snapshot must produce zero new misses");
    assert_eq!(after.hits - before.hits, 2, "both columns must be cache hits");
}

#[semio_framework_async_macros::async_test]
async fn changing_one_columns_cells_misses_only_that_columns_cache_entry() {
    let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() }).await;
    let base = two_column_snapshot();
    let _ = store::infer_field::<SemioTableSnapshot, ColumnEntropy>(&base, Some(&mut cache));

    let mut changed = base.clone();
    changed.rows[0].cells[0] = SemioValue::Str { value: "edge".into() };
    let before = cache.stats().await;
    let values = store::infer_field::<SemioTableSnapshot, ColumnEntropy>(&changed, Some(&mut cache));
    let after = cache.stats().await;

    assert_eq!(after.misses - before.misses, 1, "only coin's own entry may miss when its own cells change");
    assert_eq!(values.get("always_a").map(|e| e.distinct), Some(1), "always_a's entropy must be untouched");
}

#[semio_framework_async_macros::async_test]
async fn changing_the_other_column_misses_only_its_own_entry() {
    // 🔁️ Unlike `📊moments` (which has a non-numeric column genuinely OFF the plan to edit for a
    // zero-miss control), `entropy` tracks EVERY declared column, so this fixture has no untracked
    // column at all — the isolation law instead is: editing `always_a` misses ONLY `always_a`'s
    // own cache entry, proven in both directions together with
    // `changing_one_columns_cells_misses_only_that_columns_cache_entry` above.
    let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() }).await;
    let base = two_column_snapshot();
    let _ = store::infer_field::<SemioTableSnapshot, ColumnEntropy>(&base, Some(&mut cache));

    let mut changed = base.clone();
    changed.rows[0].cells[1] = SemioValue::Str { value: "z".into() };
    let before = cache.stats().await;
    let values = store::infer_field::<SemioTableSnapshot, ColumnEntropy>(&changed, Some(&mut cache));
    let after = cache.stats().await;
    assert_eq!(after.misses - before.misses, 1, "only always_a's own entry may miss when its own cells change");
    assert_eq!(values.get("coin").map(|e| e.distinct), Some(2), "coin's entropy must be untouched by an edit to always_a");
}
//#endregion 🧪️IncrementalityLaw
