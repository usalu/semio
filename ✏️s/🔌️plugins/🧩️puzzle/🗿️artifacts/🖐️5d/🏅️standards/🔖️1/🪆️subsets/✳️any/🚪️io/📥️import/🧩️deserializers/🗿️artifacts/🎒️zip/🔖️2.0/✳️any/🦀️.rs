//! puzzle5d ← zip — the DSL member of a document archive written by the sibling export leaf (or any
//! archive carrying that member), parsed as this artifact's own DSL (`IoFidelity::Exact`).
use crate::Puzzle5dSnapshot;
use semio_s_artifact_stdio_zip::io::decode_document_archive;

pub fn register() {}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Puzzle5dSnapshot, store::TextError> {
    decode_document_archive(bytes).map_err(|error| store::TextError::new(format!("puzzle5d←zip: {error}"), dsl::TextSpan::at(1, 1)))
}
