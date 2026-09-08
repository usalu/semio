//! 🧾 `outline` — one named inference: this RFC 4180 table's own row/column structure.
//! `recordCount` is `records.len()` verbatim; `columnCount` is the widest record's field count
//! (a real table is often ragged on the wire — this reports the true maximum, never assumes
//! rectangularity); `hasHeader` mirrors the snapshot's own `has_header` flag.

use crate::CsvSnapshot;

//#region 🔖️Outline
/// 🧾️ `Csv` document outline.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct CsvOutline {
    pub record_count: u32,
    pub column_count: u32,
    pub has_header: bool,
}

impl Default for CsvOutline {
    fn default() -> Self {
        Self { record_count: 0, column_count: 0, has_header: true }
    }
}

impl CsvOutline {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compute(snapshot: &CsvSnapshot) -> Self {
        let record_count = snapshot.records.len() as u32;
        let column_count = snapshot.records.iter().map(|r| r.fields.len() as u32).max().unwrap_or(0);
        Self { record_count, column_count, has_header: snapshot.has_header }
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
