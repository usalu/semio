//! jack -> csv
use crate::artifacts::jack::JackSnapshot;
use semio_s_plugin_stdio::artifacts::csv::schema::snapshot::{CsvField, CsvRecord};
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub fn register() {}

/// 📑 One header record (`"payload"`) plus one data record holding the printed DSL text --
/// `CsvSnapshot` is now `has_header` + index-keyed `records`, not a flat `headers`/`rows` pair.
pub fn serialize(snapshot: &JackSnapshot) -> Result<CsvSnapshot, store::TextError> {
    Ok(CsvSnapshot {
        schema: STDIO_CSV_DOCUMENT_SCHEMA.into(),
        has_header: true,
        records: vec![CsvRecord { fields: vec![CsvField { value: "payload".into(), quoted: false }] }, CsvRecord { fields: vec![CsvField { value: <JackSnapshot as store::ArtifactDsl>::print_dsl(snapshot), quoted: false }] }],
    })
}

pub fn serialize_bytes(snapshot: &JackSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<CsvSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}
