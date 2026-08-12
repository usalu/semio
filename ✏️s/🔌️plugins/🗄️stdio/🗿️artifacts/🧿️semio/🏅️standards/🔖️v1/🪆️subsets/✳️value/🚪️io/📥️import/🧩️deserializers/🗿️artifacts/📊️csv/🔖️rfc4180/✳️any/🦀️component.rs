//! 📥️ `SemioValueFromCsv` — tabular rows -> a `List` of `Map`s. A genuine SHAPE mismatch,
//! documented rather than forced smooth:
//!
//! - `has_header: true` (RFC 4180's own default): `root = List{items: one Map per DATA record}`,
//!   each `Map` keyed by the header row's field values, values always `Str` (CSV is a text-only
//!   format — every cell is a string on the wire, never structurally typed). A record with FEWER
//!   fields than the header gets the missing trailing keys omitted entirely (never fabricated as
//!   empty strings); a record with MORE fields than the header has its EXTRA trailing fields
//!   dropped (documented — there is no header key to attach them to).
//! - `has_header: false`: there is no key set at all, so the natural shape is a `List` of `List`s
//!   (`root = List{items: one List<Str> per record}`) rather than inventing positional keys
//!   (`"0"`,`"1"`,…) that would silently look like real column names.
//! - The RFC 4180 `CsvField.quoted` flag (whether the SOURCE quoted a field) has no home on a
//!   plain `Str` value and is dropped — see the serializer's own doc comment for the encode side.
//! - `nodes` always decodes empty — CSV has no graph/reference concept.

use crate::artifacts::csv::CsvSnapshot;
use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::{SemioValueEntry, SemioValueSnapshot, SemioValue, STDIO_SEMIOVALUE_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};

//#region 🔖️Deserializer
pub struct SemioValueFromCsv;

impl ArtifactDeserializer for SemioValueFromCsv {
    type From = CsvSnapshot;
    type Into = SemioValueSnapshot;
    const FROM: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId::ANY };
    const INTO: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("value") };

    fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        Ok(SemioValueSnapshot { schema: STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(), root: semio_value_from_csv(from), nodes: Vec::new() })
    }
}

pub fn register() {}
//#endregion 🔖️Deserializer

//#region 🔖️Convert
pub fn semio_value_from_csv(snapshot: &CsvSnapshot) -> SemioValue {
    if snapshot.has_header {
        let mut records = snapshot.records.iter();
        let header: Vec<String> = records.next().map(|r| r.fields.iter().map(|f| f.value.clone()).collect()).unwrap_or_default();
        let items = records
            .map(|record| {
                let entries = header
                    .iter()
                    .zip(record.fields.iter())
                    .map(|(key, field)| SemioValueEntry { key: key.clone(), value: SemioValue::Str { value: field.value.clone() } })
                    .collect();
                SemioValue::Map { entries }
            })
            .collect();
        SemioValue::List { items }
    } else {
        let items = snapshot
            .records
            .iter()
            .map(|record| SemioValue::List { items: record.fields.iter().map(|f| SemioValue::Str { value: f.value.clone() }).collect() })
            .collect();
        SemioValue::List { items }
    }
}
//#endregion 🔖️Convert

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::csv::schema::snapshot::{CsvField, CsvRecord};

    fn field(s: &str) -> CsvField {
        CsvField { value: s.into(), quoted: false }
    }

    #[test]
    fn header_rows_become_a_list_of_keyed_maps() {
        let snapshot = CsvSnapshot {
            schema: crate::artifacts::csv::STDIO_CSV_DOCUMENT_SCHEMA.into(),
            has_header: true,
            records: vec![
                CsvRecord { fields: vec![field("name"), field("age")] },
                CsvRecord { fields: vec![field("Ada"), field("36")] },
                CsvRecord { fields: vec![field("Grace"), field("85")] },
            ],
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

    #[test]
    fn ragged_short_record_omits_missing_trailing_keys() {
        let snapshot = CsvSnapshot {
            schema: crate::artifacts::csv::STDIO_CSV_DOCUMENT_SCHEMA.into(),
            has_header: true,
            records: vec![CsvRecord { fields: vec![field("a"), field("b"), field("c")] }, CsvRecord { fields: vec![field("1")] }],
        };
        let value = semio_value_from_csv(&snapshot);
        match value {
            SemioValue::List { items } => match &items[0] {
                SemioValue::Map { entries } => assert_eq!(entries.len(), 1, "only the header key present in the short record survives"),
                other => panic!("expected map, got {other:?}"),
            },
            other => panic!("expected list, got {other:?}"),
        }
    }

    #[test]
    fn headerless_csv_becomes_a_list_of_lists() {
        let snapshot = CsvSnapshot {
            schema: crate::artifacts::csv::STDIO_CSV_DOCUMENT_SCHEMA.into(),
            has_header: false,
            records: vec![CsvRecord { fields: vec![field("x"), field("y")] }],
        };
        let value = semio_value_from_csv(&snapshot);
        match value {
            SemioValue::List { items } => match &items[0] {
                SemioValue::List { items } => assert_eq!(items, &vec![SemioValue::Str { value: "x".into() }, SemioValue::Str { value: "y".into() }]),
                other => panic!("expected nested list, got {other:?}"),
            },
            other => panic!("expected list, got {other:?}"),
        }
    }
}
//#endregion 🧪️Tests
