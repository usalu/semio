//! imperative <- csv
use crate::artifacts::imperative::schema::snapshot::ImperativeSnapshot;
use semio_s_plugin_stdio::artifacts::csv::{CsvSnapshot, STDIO_CSV_DOCUMENT_SCHEMA};

pub async fn register() {}

/// 🩹️ `stdio_gap` fix (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE, found while verifying
/// the W1b `.artifact()` conversion — `CsvSnapshot` moved from a flat `headers`/`rows` pair to
/// `has_header` + index-keyed `records: Vec<CsvRecord>` by a concurrent stdio wave; imperative's
/// old body never matched a real `ImperativeSnapshot` shape anyway. Mirrors `🔱️jack`'s own stub
/// for the same reason: an arbitrary CSV grid has no tabular correspondence to a `Path` of
/// `Step`s to reconstruct.
pub async fn deserialize(from: &CsvSnapshot) -> Result<ImperativeSnapshot, store::TextError> {
    let _ = (STDIO_CSV_DOCUMENT_SCHEMA, from);
    Ok(ImperativeSnapshot::default())
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<ImperativeSnapshot, store::TextError> {
    <ImperativeSnapshot as store::ArtifactPack>::decode_pack(bytes)
        .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}
