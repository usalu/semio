//! 🧾 `outline` — one named inference: this IANA TSV table's own row/column structure.
//! `recordCount` is `records.len()` verbatim; `columnCount` is the widest record's cell count (a
//! real TSV file is often ragged on the wire — IANA TSV draws no header/data structural
//! distinction, so this never assumes rectangularity or a header row).

use crate::TsvSnapshot;

//#region 🔖️Outline
/// 🧾️ `Tsv` document outline.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct TsvOutline {
    pub record_count: u32,
    pub column_count: u32,
}

impl TsvOutline {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compute(snapshot: &TsvSnapshot) -> Self {
        let record_count = snapshot.records.len() as u32;
        let column_count = snapshot.records.iter().map(|r| r.len() as u32).max().unwrap_or(0);
        Self { record_count, column_count }
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
