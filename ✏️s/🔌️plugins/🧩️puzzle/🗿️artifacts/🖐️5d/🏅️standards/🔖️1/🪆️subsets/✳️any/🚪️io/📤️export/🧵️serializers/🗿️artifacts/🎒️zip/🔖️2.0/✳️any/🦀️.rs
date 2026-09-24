//! puzzle5d → zip — the shared document archive (`encode_document_archive`): this artifact's DSL as the
//! authoritative member plus its rfc8259 rendition, both lossless (`IoFidelity::Exact`).
use crate::Puzzle5dSnapshot;
use semio_s_artifact_stdio_zip::io::encode_document_archive;

pub fn register() {}

pub fn serialize_bytes(snapshot: &Puzzle5dSnapshot) -> Result<Vec<u8>, store::TextError> {
    encode_document_archive(snapshot).map_err(|error| store::TextError::new(format!("puzzle5d→zip: {error}"), dsl::TextSpan::at(1, 1)))
}
