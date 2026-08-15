//! 🎲 `entropy` — real per-column Shannon entropy (bits) of the value distribution, computed as a
//! genuine `InferredField<SemioTableSnapshot>` (not a bare pass-through): one step per DECLARED
//! column (any `SemioTableCellKind`, not just numeric — unlike `📊moments`, entropy is defined over
//! any discrete symbol alphabet), keyed by column NAME, NO parents — a column's entropy depends only
//! on its OWN cell values, mirroring `📊moments`'s per-column chain shape.
//!
//! Wraps `🎲️entropy-internals::estimators::entropy_discrete` (plug-in Shannon estimator) — moved
//! verbatim from `🧰️framework/🔨️modules/🧮️math/🎲️entropy` in ticket 26/08/12/
//! DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave M3e. Per that wave's placement
//! reasoning: entropy measures are DERIVATIONS over data already held by a subset (here, `✳️table`'s
//! own column values), not a new persisted content shape — so this is an inference over the existing
//! `✳️table` subset, not a new stdio subset.
//!
//! NOT YET wired into the parent `SemioTableInference` aggregate struct or its hand-rolled
//! binary/text/json/proto/graphql codecs, same honest remainder as `📊moments`. This field is real
//! and independently tested via `store::infer_field` directly.

use crate::artifacts::semio::standards::v1::subsets::table::schema::entropy_internals::estimators::{entropy_discrete, DiscreteMethod};
use crate::artifacts::semio::standards::v1::subsets::table::schema::entropy_internals::LogBase;
use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;
use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValue;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Value
/// 🎲 One column's Shannon entropy (bits) over its own non-null cell values, treated as a discrete
/// symbol alphabet. `SemioColumnEntropy::default()` (all-zero) is the honest "no data" value for a
/// column with zero non-null cells, same convention `SemioColumnMoments::default()` uses.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioColumnEntropy {
    /// 🎲 Non-null cells that contributed a symbol.
    pub count: u32,
    /// 🎲 Distinct symbols observed among those cells.
    pub distinct: u32,
    /// 🎲 Plug-in Shannon entropy of the symbol distribution, in bits.
    pub bits: f64,
}
//#endregion 🔖️Value

//#region 🔖️Lookup
/// 🔎 A cell's canonical discrete symbol for entropy purposes — `Null` contributes no symbol (honest
/// "missing", not a fabricated category); every other kind's own textual form is its symbol, so
/// `Int`/`Float`/`Bool`/`Str` are each counted by their own printed identity. `Bytes`/`List`/`Map`/
/// `Ref` have no stable scalar identity and are excluded, exactly as non-numeric cells are excluded
/// from `📊moments`.
fn cell_symbol(cell: &SemioValue) -> Option<String> {
    match cell {
        SemioValue::Null => None,
        SemioValue::Bool { value } => Some(value.to_string()),
        SemioValue::Int { lexeme } => Some(lexeme.clone()),
        SemioValue::Float { lexeme } => Some(lexeme.clone()),
        SemioValue::Str { value } => Some(value.clone()),
        SemioValue::Bytes { .. } | SemioValue::List { .. } | SemioValue::Map { .. } | SemioValue::Ref { .. } => None,
    }
}

/// 🔎 Positionally aligned column-name → symbol-occurrence-counts, in symbol-sorted (`BTreeMap`)
/// order so the resulting `Vec<u64>` — and therefore `dep_input`'s serialized bytes — is deterministic
/// across processes, never dependent on `HashMap` iteration order (a hard requirement: `DepHash`
/// caching is only sound if `dep_input` is a deterministic function of the snapshot).
fn column_symbol_counts(snapshot: &SemioTableSnapshot, column_name: &str) -> Vec<u64> {
    let Some(idx) = snapshot.columns.iter().position(|c| c.name == column_name) else {
        return Vec::new();
    };
    let mut counts: BTreeMap<String, u64> = BTreeMap::new();
    for row in &snapshot.rows {
        if let Some(symbol) = row.cells.get(idx).and_then(cell_symbol) {
            *counts.entry(symbol).or_insert(0) += 1;
        }
    }
    counts.into_values().collect()
}
//#endregion 🔖️Lookup

//#region 🔖️DependencyHashChain
pub struct ColumnEntropy;

impl store::InferredField<SemioTableSnapshot> for ColumnEntropy {
    type Key = String;
    type Value = SemioColumnEntropy;
    const FIELD_ID: &'static str = "s.stdio.semio.table.inference.entropy";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        &["columns", "rows"]
    }

    fn plan(snapshot: &SemioTableSnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        snapshot.columns.iter().map(|c| store::InferenceStep { key: c.name.clone(), parents: Vec::new() }).collect()
    }

    /// 🔑 Canonical dependency-input bytes — EXACTLY this column's own symbol-occurrence counts, in
    /// deterministic sorted-symbol order, nothing else — an unrelated column's edit must still hit
    /// the cache, proven by the incrementality-law test below.
    fn dep_input(snapshot: &SemioTableSnapshot, key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        serde_json::to_vec(&column_symbol_counts(snapshot, key)).unwrap_or_default()
    }

    fn compute(snapshot: &SemioTableSnapshot, key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        let counts = column_symbol_counts(snapshot, key);
        let count = counts.iter().sum::<u64>() as u32;
        let distinct = counts.len() as u32;
        let bits = entropy_discrete(&counts, DiscreteMethod::Plugin, LogBase::Bits).map(|est| est.value).unwrap_or(0.0);
        SemioColumnEntropy { count, distinct, bits }
    }
}
//#endregion 🔖️DependencyHashChain

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::{SemioTableCellKind, SemioTableColumn, SemioTableRow, STDIO_SEMIOTABLE_DOCUMENT_SCHEMA};
    use store::{InferenceCache, InferenceCacheConfig, InferredField};

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
    #[test]
    fn a_fair_binary_column_has_one_bit_of_entropy() {
        let values = store::infer_field::<SemioTableSnapshot, ColumnEntropy>(&two_column_snapshot(), None);
        let e = values.get("coin").expect("coin entropy present");
        assert_eq!(e.count, 4);
        assert_eq!(e.distinct, 2);
        assert!((e.bits - 1.0).abs() < 1e-9, "fair coin must be exactly 1 bit, got {}", e.bits);
    }

    #[test]
    fn a_constant_column_has_zero_entropy() {
        let values = store::infer_field::<SemioTableSnapshot, ColumnEntropy>(&two_column_snapshot(), None);
        let e = values.get("always_a").expect("always_a entropy present");
        assert_eq!(e.distinct, 1);
        assert!(e.bits.abs() < 1e-9, "single-symbol column must have zero entropy, got {}", e.bits);
    }

    #[test]
    fn every_declared_column_appears_regardless_of_kind() {
        let values = store::infer_field::<SemioTableSnapshot, ColumnEntropy>(&two_column_snapshot(), None);
        assert_eq!(values.len(), 2, "entropy is defined over any symbol alphabet, unlike moments' numeric-only gate");
    }

    #[test]
    fn an_all_empty_snapshot_yields_an_empty_plan() {
        let values = store::infer_field::<SemioTableSnapshot, ColumnEntropy>(&SemioTableSnapshot::default(), None);
        assert!(values.is_empty());
    }
    //#endregion 🧪️Honesty

    //#region 🧪️CacheTransparencyLaw
    #[test]
    fn disabled_cache_matches_pure_recompute() {
        let snapshot = two_column_snapshot();
        let pure = store::infer_field::<SemioTableSnapshot, ColumnEntropy>(&snapshot, None);
        let mut disabled = InferenceCache::new(InferenceCacheConfig { enabled: false, ..Default::default() });
        let via_disabled = store::infer_field::<SemioTableSnapshot, ColumnEntropy>(&snapshot, Some(&mut disabled));
        assert_eq!(pure, via_disabled);
    }
    //#endregion 🧪️CacheTransparencyLaw

    //#region 🧪️IncrementalityLaw
    #[test]
    fn identical_snapshot_recompute_is_a_cache_hit() {
        let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() });
        let base = two_column_snapshot();
        let _ = store::infer_field::<SemioTableSnapshot, ColumnEntropy>(&base, Some(&mut cache));
        let before = cache.stats();
        let _ = store::infer_field::<SemioTableSnapshot, ColumnEntropy>(&base, Some(&mut cache));
        let after = cache.stats();
        assert_eq!(after.misses, before.misses, "an unchanged snapshot must produce zero new misses");
        assert_eq!(after.hits - before.hits, 2, "both columns must be cache hits");
    }

    #[test]
    fn changing_one_columns_cells_misses_only_that_columns_cache_entry() {
        let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() });
        let base = two_column_snapshot();
        let _ = store::infer_field::<SemioTableSnapshot, ColumnEntropy>(&base, Some(&mut cache));

        let mut changed = base.clone();
        changed.rows[0].cells[0] = SemioValue::Str { value: "edge".into() };
        let before = cache.stats();
        let values = store::infer_field::<SemioTableSnapshot, ColumnEntropy>(&changed, Some(&mut cache));
        let after = cache.stats();

        assert_eq!(after.misses - before.misses, 1, "only coin's own entry may miss when its own cells change");
        assert_eq!(values.get("always_a").map(|e| e.distinct), Some(1), "always_a's entropy must be untouched");
    }

    #[test]
    fn changing_the_other_column_misses_only_its_own_entry() {
        // 🔁️ Unlike `📊moments` (which has a non-numeric column genuinely OFF the plan to edit for a
        // zero-miss control), `entropy` tracks EVERY declared column, so this fixture has no untracked
        // column at all — the isolation law instead is: editing `always_a` misses ONLY `always_a`'s
        // own cache entry, proven in both directions together with
        // `changing_one_columns_cells_misses_only_that_columns_cache_entry` above.
        let mut cache = InferenceCache::new(InferenceCacheConfig { enabled: true, record_stats: true, ..Default::default() });
        let base = two_column_snapshot();
        let _ = store::infer_field::<SemioTableSnapshot, ColumnEntropy>(&base, Some(&mut cache));

        let mut changed = base.clone();
        changed.rows[0].cells[1] = SemioValue::Str { value: "z".into() };
        let before = cache.stats();
        let values = store::infer_field::<SemioTableSnapshot, ColumnEntropy>(&changed, Some(&mut cache));
        let after = cache.stats();
        assert_eq!(after.misses - before.misses, 1, "only always_a's own entry may miss when its own cells change");
        assert_eq!(values.get("coin").map(|e| e.distinct), Some(2), "coin's entropy must be untouched by an edit to always_a");
    }
    //#endregion 🧪️IncrementalityLaw
}
//#endregion 🧪️Tests
