//! home <- csv
use crate::artifacts::home::schema::snapshot::SHomeSnapshot;
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub async fn register() {}

/// 🌉 `SHomeSnapshot` requires `schema`/`catalogGeneration`, neither of which a bare csv
/// table carries — this bridge was always a hard failure regardless of csv content, even
/// before `CsvSnapshot` dropped `headers`/`rows` for `has_header`/`records` (stdio's own
/// RFC4180 rework). Preserved verbatim under the new shape rather than inventing a mapping
/// this snapshot pair never had.
pub async fn deserialize(from: &CsvSnapshot) -> Result<SHomeSnapshot, store::TextError> {
    let _ = STDIO_CSV_DOCUMENT_SCHEMA;
    let value = dsl::DslValue::object([("hasHeader".to_string(), dsl::ToValue::to_value(&from.has_header)), ("records".to_string(), dsl::ToValue::to_value(&from.records))]);
    dsl::FromValue::from_value(value).map_err(|e: dsl::ValueError| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<SHomeSnapshot, store::TextError> {
    <SHomeSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}
