
use super::*;
use crate::standards::v1::subsets::table::schema::snapshot::{STDIO_SEMIOTABLE_DOCUMENT_SCHEMA, SemioTableColumn, SemioTableRow};
use store::{InferenceCache, InferenceCacheConfig};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn two_numeric_column_snapshot() -> SemioTableSnapshot {
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
    let mut disabled = InferenceCache::new(InferenceCacheConfig { enabled: false, ..Default::default() }).await;
    let via_disabled = store::infer_field::<SemioTableSnapshot, ColumnMoments>(&snapshot, Some(&mut disabled));
    assert_eq!(pure, via_disabled);
}
//#endregion 🧪️CacheTransparencyLaw

//#region 🧪️IncrementalityLaw
#[semio_framework_async_macros::async_test]
async fn identical_snapshot_recompute_is_a_cache_hit() {
    let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() }).await;
    let base = two_numeric_column_snapshot();
    let _ = store::infer_field::<SemioTableSnapshot, ColumnMoments>(&base, Some(&mut cache));
    let before = cache.stats().await;
    let _ = store::infer_field::<SemioTableSnapshot, ColumnMoments>(&base, Some(&mut cache));
    let after = cache.stats().await;
    assert_eq!(after.misses, before.misses, "an unchanged snapshot must produce zero new misses");
    assert_eq!(after.hits - before.hits, 2, "both numeric columns must be cache hits");
}

#[semio_framework_async_macros::async_test]
async fn changing_one_columns_cells_misses_only_that_columns_cache_entry() {
    let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() }).await;
    let base = two_numeric_column_snapshot();
    let _ = store::infer_field::<SemioTableSnapshot, ColumnMoments>(&base, Some(&mut cache));

    let mut changed = base.clone();
    changed.rows[0].cells[0] = SemioValue::Float { lexeme: "99.0".into() };
    let before = cache.stats().await;
    let values = store::infer_field::<SemioTableSnapshot, ColumnMoments>(&changed, Some(&mut cache));
    let after = cache.stats().await;

    assert_eq!(after.misses - before.misses, 1, "only score's own entry may miss when its own cells change");
    assert_eq!(values.get("count").map(|m| m.count), Some(3), "count column's moments must be untouched");
}

#[semio_framework_async_macros::async_test]
async fn changing_an_unrelated_column_does_not_miss() {
    let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() }).await;
    let base = two_numeric_column_snapshot();
    let _ = store::infer_field::<SemioTableSnapshot, ColumnMoments>(&base, Some(&mut cache));

    let mut changed = base.clone();
    changed.rows[0].cells[2] = SemioValue::Str { value: "z".into() };
    let before = cache.stats().await;
    let _ = store::infer_field::<SemioTableSnapshot, ColumnMoments>(&changed, Some(&mut cache));
    let after = cache.stats().await;
    assert_eq!(after.misses, before.misses, "the label column has no bearing on score/count dep chains");
}
//#endregion 🧪️IncrementalityLaw
