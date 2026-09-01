//! 🧾 `outline` — one named inference: this IANA TSV table's own row/column structure.
//! `recordCount` is `records.len()` verbatim; `columnCount` is the widest record's cell count (a
//! real TSV file is often ragged on the wire — IANA TSV draws no header/data structural
//! distinction, so this never assumes rectangularity or a header row).

use crate::artifacts::tsv::TsvSnapshot;

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
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn reports_widest_record_as_column_count() {
        let snapshot = TsvSnapshot { schema: "stdio.tsv".into(), records: vec![vec!["a".into(), "b".into()], vec!["c".into()]], trailing_newline: false, line_ending: Default::default() };
        let outline = TsvOutline::compute(&snapshot);
        assert_eq!(outline.record_count, 2);
        assert_eq!(outline.column_count, 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn outline_is_deterministic() {
        let snapshot = TsvSnapshot::default();
        assert_eq!(TsvOutline::compute(&snapshot), TsvOutline::compute(&snapshot));
    }
}
//#endregion 🧪️Tests
