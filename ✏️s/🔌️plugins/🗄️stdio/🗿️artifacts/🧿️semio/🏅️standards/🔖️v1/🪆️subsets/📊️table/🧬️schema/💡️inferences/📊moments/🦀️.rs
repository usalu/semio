//! 📊 `moments` — real per-column descriptive moments, computed as a genuine
//! `InferredField<SemioTableSnapshot>` (not a bare pass-through): one step per NUMERIC (`Int`/
//! `Float`) column, keyed by column NAME (the native key, per this subset's own `SemioTableColumn`
//! doc comment), NO parents — a column's moments depend only on its OWN cell values, never on any
//! other column's, mirroring `🔺️mesh`'s `📦aabb` pilot's per-primitive chain shape.
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

use crate::standards::v1::subsets::table::schema::snapshot::{SemioTableCellKind, SemioTableSnapshot};
use crate::standards::v1::subsets::table::schema::statistics_internals;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValue;

//#region 🔖️Value
/// 📊️ One numeric column's descriptive moments. `SemioColumnMoments::default()` (all-zero) is the
/// honest "no numeric data" value for a column with zero parseable cells — same convention
/// `🔺️mesh`'s `SemioAabb::default()` uses for "no geometry".
/// 🔀️ No longer dual-derives `serde`: `store::InferredField::Value` used to bound on `Serialize +
/// DeserializeOwned`, forcing every implementor onto serde regardless of its own fields — that
/// bound now reads `ToValue + FromValue` (ticket
/// `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`), so this leaf drops the
/// serde half entirely.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn numeric_cell_value(cell: &SemioValue) -> Option<f64> {
    match cell {
        SemioValue::Int { lexeme } => lexeme.parse::<f64>().ok(),
        SemioValue::Float { lexeme } => lexeme.parse::<f64>().ok(),
        _ => None,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn column_values(snapshot: &SemioTableSnapshot, column_name: &str) -> Vec<f64> {
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

    fn reads() -> &'static [&'static str] {
        &["columns", "rows"]
    }

    fn plan(snapshot: &SemioTableSnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        snapshot.columns.iter().filter(|c| matches!(c.kind, SemioTableCellKind::Int | SemioTableCellKind::Float)).map(|c| store::InferenceStep { key: c.name.clone(), parents: Vec::new() }).collect()
    }

    /// 🔑 Canonical dependency-input bytes — EXACTLY this column's own numeric cell values, nothing
    /// else (not other columns, not `kind`, which `plan` already gates on) — an unrelated column's
    /// edit must still hit the cache, proven by the incrementality-law test below.
    fn dep_input(snapshot: &SemioTableSnapshot, key: &Self::Key, _parents: &[Self::Key]) -> Vec<u8> {
        pack::to_json_string(&column_values(snapshot, key)).into_bytes()
    }

    fn compute(snapshot: &SemioTableSnapshot, key: &Self::Key, _parents: &[Self::Value]) -> Self::Value {
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
