//! home -> csv
use crate::artifacts::home::schema::snapshot::SHomeSnapshot;
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub async fn register() {}

/// 🌉 `SHomeSnapshot` has no `headers`/`rows`-shaped fields (its own JSON shape is
/// `{schema, catalogGeneration}`), so this bridge was always degenerate — an empty table,
/// regardless of snapshot content — even before `CsvSnapshot` dropped `headers`/`rows` for
/// `has_header`/`records` (stdio's own RFC4180 rework). Preserved verbatim under the new shape
/// rather than inventing table content this snapshot was never able to carry.
pub async fn serialize(snapshot: &SHomeSnapshot) -> Result<CsvSnapshot, store::TextError> {
    let _ = dsl::ToValue::to_value(snapshot);
    Ok(CsvSnapshot { schema: STDIO_CSV_DOCUMENT_SCHEMA.into(), has_header: true, records: Vec::new() })
}

pub async fn serialize_bytes(snapshot: &SHomeSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<CsvSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}
