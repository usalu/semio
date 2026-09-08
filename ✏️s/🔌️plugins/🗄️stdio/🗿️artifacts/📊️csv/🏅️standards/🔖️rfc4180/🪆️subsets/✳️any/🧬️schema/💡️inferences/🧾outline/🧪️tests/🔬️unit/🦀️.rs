
use super::*;
use crate::schema::snapshot::{CsvField, CsvRecord};

#[semio_framework_async_macros::async_test]
async fn reports_widest_record_as_column_count() {
    let snapshot = CsvSnapshot {
        schema: "stdio.csv".into(),
        has_header: true,
        records: vec![CsvRecord { fields: vec![CsvField { value: "a".into(), quoted: false }, CsvField { value: "b".into(), quoted: false }] }, CsvRecord { fields: vec![CsvField { value: "c".into(), quoted: false }] }],
    };
    let outline = CsvOutline::compute(&snapshot);
    assert_eq!(outline.record_count, 2);
    assert_eq!(outline.column_count, 2);
    assert!(outline.has_header);
}

#[semio_framework_async_macros::async_test]
async fn outline_is_deterministic() {
    let snapshot = CsvSnapshot::default();
    assert_eq!(CsvOutline::compute(&snapshot), CsvOutline::compute(&snapshot));
}
