//! 🎲 `entropy` — real per-column Shannon entropy (bits) of the value distribution, computed as a
//! genuine `InferredField<SemioTableSnapshot>` (not a bare pass-through): one step per DECLARED
//! column (any `SemioTableCellKind`, not just numeric — unlike `📊moments`, entropy is defined over
//! any discrete symbol alphabet), keyed by column NAME, NO parents — a column's entropy depends only
//! on its OWN cell values, mirroring `📊moments`'s per-column chain shape.
//!
//! Wraps `🌀️entropy-internals::estimators::entropy_discrete` (plug-in Shannon estimator) — moved
//! verbatim from `🧰️framework/🔨️modules/🧮️math/🎲️entropy` in ticket 26/08/12/
//! DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave M3e. Per that wave's placement
//! reasoning: entropy measures are DERIVATIONS over data already held by a subset (here, `📊️table`'s
//! own column values), not a new persisted content shape — so this is an inference over the existing
//! `📊️table` subset, not a new stdio subset.
//!
//! NOT YET wired into the parent `SemioTableInference` aggregate struct or its hand-rolled
//! binary/text/json/proto/graphql codecs, same honest remainder as `📊moments`. This field is real
//! and independently tested via `store::infer_field` directly.

use crate::standards::v1::subsets::table::schema::entropy_internals::estimators::{entropy_discrete, DiscreteMethod};
use crate::standards::v1::subsets::table::schema::entropy_internals::LogBase;
use crate::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValue;
use std::collections::BTreeMap;

//#region 🔖️Value
/// 🎲 One column's Shannon entropy (bits) over its own non-null cell values, treated as a discrete
/// symbol alphabet. `SemioColumnEntropy::default()` (all-zero) is the honest "no data" value for a
/// column with zero non-null cells, same convention `SemioColumnMoments::default()` uses.
/// 🔀️ No longer dual-derives `serde`: `store::InferredField::Value` used to bound on `Serialize +
/// DeserializeOwned`, forcing every implementor onto serde regardless of its own fields — that
/// bound now reads `ToValue + FromValue` (ticket
/// `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`), so this leaf drops the
/// serde half entirely.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
        pack::to_json_string(&column_symbol_counts(snapshot, key)).into_bytes()
    }

    fn compute(snapshot: &SemioTableSnapshot, key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
        let counts = column_symbol_counts(snapshot, key);
        let count = counts.iter().sum::<u64>() as u32;
        let distinct = counts.len() as u32;
        let bits = entropy_discrete(&counts, DiscreteMethod::Plugin, LogBase::Bits).map_or(0.0, |est| est.value);
        SemioColumnEntropy { count, distinct, bits }
    }
}
//#endregion 🔖️DependencyHashChain

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
