
use super::*;
use crate::standards::v1::subsets::value::io::import::deserializers::artifacts::csv::v_rfc4180::any::semio_value_from_csv;
use semio_s_artifact_stdio_csv::schema::snapshot::CsvField as CsvFieldT;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn field(s: &str) -> CsvFieldT {
    CsvFieldT { value: s.into(), quoted: false }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn round_trip(snapshot: &CsvSnapshot) -> CsvSnapshot {
    let value = semio_value_from_csv(snapshot);
    let nodes = HashMap::new();
    let mut visiting = HashSet::new();
    csv_from_semio(&value, &nodes, &mut visiting).expect("value->csv")
}

/// 🧪️ Required proof: csv -> value -> csv -> value round trip preserves everything the
/// value subset can represent (the `quoted` flag excepted — documented lossy field).
#[semio_framework_async_macros::async_test]
async fn csv_to_value_to_csv_to_value_round_trips() {
    let snapshot = CsvSnapshot {
        schema: STDIO_CSV_DOCUMENT_SCHEMA.into(),
        has_header: true,
        records: vec![CsvRecord { fields: vec![field("name"), field("age"), field("city")] }, CsvRecord { fields: vec![field("Ada"), field("36"), field("London")] }, CsvRecord { fields: vec![field("Grace"), field("85"), field("New York")] }],
    };
    let s1 = semio_value_from_csv(&snapshot);
    let csv_x = round_trip(&snapshot);
    let s2 = semio_value_from_csv(&csv_x);
    assert_eq!(s1, s2);
    assert_eq!(csv_x.has_header, true);
    assert_eq!(csv_x.records.len(), 3);
}

#[semio_framework_async_macros::async_test]
async fn headerless_round_trips() {
    let snapshot = CsvSnapshot { schema: STDIO_CSV_DOCUMENT_SCHEMA.into(), has_header: false, records: vec![CsvRecord { fields: vec![field("x"), field("y")] }, CsvRecord { fields: vec![field("1"), field("2")] }] };
    let csv_x = round_trip(&snapshot);
    assert!(!csv_x.has_header);
    assert_eq!(csv_x.records, snapshot.records);
}

#[semio_framework_async_macros::async_test]
async fn mismatched_row_shape_is_a_hard_error() {
    let value = SemioValue::List {
        items: vec![
            SemioValue::Map { entries: vec![crate::standards::v1::subsets::value::schema::snapshot::SemioValueEntry { key: "a".into(), value: SemioValue::Str { value: "1".into() } }] },
            SemioValue::Map { entries: vec![crate::standards::v1::subsets::value::schema::snapshot::SemioValueEntry { key: "b".into(), value: SemioValue::Str { value: "2".into() } }] },
        ],
    };
    let nodes = HashMap::new();
    let mut visiting = HashSet::new();
    assert!(csv_from_semio(&value, &nodes, &mut visiting).is_err());
}

#[semio_framework_async_macros::async_test]
async fn nested_container_cell_is_a_hard_error() {
    let value = SemioValue::List { items: vec![SemioValue::List { items: vec![SemioValue::List { items: vec![] }] }] };
    let nodes = HashMap::new();
    let mut visiting = HashSet::new();
    assert!(csv_from_semio(&value, &nodes, &mut visiting).is_err());
}
