//! 🧾 `outline` — one named inference: this RFC 4180 table's own row/column structure.
//! `recordCount` is `records.len()` verbatim; `columnCount` is the widest record's field count
//! (a real table is often ragged on the wire — this reports the true maximum, never assumes
//! rectangularity); `hasHeader` mirrors the snapshot's own `has_header` flag.

use crate::artifacts::csv::CsvSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Outline
/// 🧾️ `Csv` document outline.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CsvOutline {
    pub record_count: u32,
    pub column_count: u32,
    pub has_header: bool,
}

impl CsvOutline {
    pub fn compute(snapshot: &CsvSnapshot) -> Self {
        let record_count = snapshot.records.len() as u32;
        let column_count = snapshot.records.iter().map(|r| r.fields.len() as u32).max().unwrap_or(0);
        Self { record_count, column_count, has_header: snapshot.has_header }
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::csv::schema::snapshot::{CsvField, CsvRecord};

    #[test]
    fn reports_widest_record_as_column_count() {
        let snapshot = CsvSnapshot {
            schema: "stdio.csv".into(),
            has_header: true,
            records: vec![
                CsvRecord { fields: vec![CsvField { value: "a".into(), quoted: false }, CsvField { value: "b".into(), quoted: false }] },
                CsvRecord { fields: vec![CsvField { value: "c".into(), quoted: false }] },
            ],
        };
        let outline = CsvOutline::compute(&snapshot);
        assert_eq!(outline.record_count, 2);
        assert_eq!(outline.column_count, 2);
        assert!(outline.has_header);
    }

    #[test]
    fn outline_is_deterministic() {
        let snapshot = CsvSnapshot::default();
        assert_eq!(CsvOutline::compute(&snapshot), CsvOutline::compute(&snapshot));
    }
}
//#endregion 🧪️Tests
