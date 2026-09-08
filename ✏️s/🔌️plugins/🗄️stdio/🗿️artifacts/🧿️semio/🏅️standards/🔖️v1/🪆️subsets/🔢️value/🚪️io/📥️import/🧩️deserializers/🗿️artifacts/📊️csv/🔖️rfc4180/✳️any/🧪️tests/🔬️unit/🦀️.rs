
use super::*;
use semio_s_artifact_stdio_csv::schema::snapshot::{CsvField, CsvRecord};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn field(s: &str) -> CsvField {
    CsvField { value: s.into(), quoted: false }
}

#[semio_framework_async_macros::async_test]
async fn header_rows_become_a_list_of_keyed_maps() {
    let snapshot = CsvSnapshot {
        schema: semio_s_artifact_stdio_csv::STDIO_CSV_DOCUMENT_SCHEMA.into(),
        has_header: true,
        records: vec![CsvRecord { fields: vec![field("name"), field("age")] }, CsvRecord { fields: vec![field("Ada"), field("36")] }, CsvRecord { fields: vec![field("Grace"), field("85")] }],
    };
    let value = semio_value_from_csv(&snapshot);
    match value {
        SemioValue::List { items } => {
            assert_eq!(items.len(), 2);
            assert_eq!(items[0], SemioValue::Map { entries: vec![SemioValueEntry { key: "name".into(), value: SemioValue::Str { value: "Ada".into() } }, SemioValueEntry { key: "age".into(), value: SemioValue::Str { value: "36".into() } }] });
        }
        other => panic!("expected list, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn ragged_short_record_omits_missing_trailing_keys() {
    let snapshot = CsvSnapshot { schema: semio_s_artifact_stdio_csv::STDIO_CSV_DOCUMENT_SCHEMA.into(), has_header: true, records: vec![CsvRecord { fields: vec![field("a"), field("b"), field("c")] }, CsvRecord { fields: vec![field("1")] }] };
    let value = semio_value_from_csv(&snapshot);
    match value {
        SemioValue::List { items } => match &items[0] {
            SemioValue::Map { entries } => assert_eq!(entries.len(), 1, "only the header key present in the short record survives"),
            other => panic!("expected map, got {other:?}"),
        },
        other => panic!("expected list, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn headerless_csv_becomes_a_list_of_lists() {
    let snapshot = CsvSnapshot { schema: semio_s_artifact_stdio_csv::STDIO_CSV_DOCUMENT_SCHEMA.into(), has_header: false, records: vec![CsvRecord { fields: vec![field("x"), field("y")] }] };
    let value = semio_value_from_csv(&snapshot);
    match value {
        SemioValue::List { items } => match &items[0] {
            SemioValue::List { items } => assert_eq!(items, &vec![SemioValue::Str { value: "x".into() }, SemioValue::Str { value: "y".into() }]),
            other => panic!("expected nested list, got {other:?}"),
        },
        other => panic!("expected list, got {other:?}"),
    }
}
