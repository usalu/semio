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

use crate::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry, SemioValueSnapshot, STDIO_SEMIOVALUE_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_csv::CsvSnapshot;

//#region 🔖️Deserializer
pub struct SemioValueFromCsv;

impl ArtifactDeserializer for SemioValueFromCsv {
    type From = CsvSnapshot;
    type Into = SemioValueSnapshot;
    const FROM: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId::ANY };
    const INTO: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("value") };

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        Ok(SemioValueSnapshot { schema: STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(), root: semio_value_from_csv(from), nodes: Vec::new() })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn register() {}
//#endregion 🔖️Deserializer

//#region 🔖️Convert
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_value_from_csv(snapshot: &CsvSnapshot) -> SemioValue {
    if snapshot.has_header {
        let mut records = snapshot.records.iter();
        let header: Vec<String> = records.next().map(|r| r.fields.iter().map(|f| f.value.clone()).collect()).unwrap_or_default();
        let items = records
            .map(|record| {
                let entries = header.iter().zip(record.fields.iter()).map(|(key, field)| SemioValueEntry { key: key.clone(), value: SemioValue::Str { value: field.value.clone() } }).collect();
                SemioValue::Map { entries }
            })
            .collect();
        SemioValue::List { items }
    } else {
        let items = snapshot.records.iter().map(|record| SemioValue::List { items: record.fields.iter().map(|f| SemioValue::Str { value: f.value.clone() }).collect() }).collect();
        SemioValue::List { items }
    }
}
//#endregion 🔖️Convert

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
