//! 🚪️ curation <- png — foreign `Deserializer<CurationSnapshot>` (ticket 26/08/17/CLEAN-ARTIFACT-
//! STANDARD-SUBSET-MECHANISM design.md §3).
//!
//! ⚠️ Pre-existing, not fixed this pass: `deserialize_bytes` decodes incoming bytes directly as a
//! `CurationSnapshot` pack rather than as a `PngSnapshot` pack, and `deserialize(from: &PngSnapshot)`
//! merely re-encodes `from` and forwards into that same (format-mismatched) path — confirmed by
//! inspection, not a regression introduced by this pass. No domain-correct "PNG image -> curation
//! catalogue" mapping is defined anywhere in this codebase (an image carries no catalogue-object
//! semantics), so both functions are preserved byte-for-byte and honestly labeled
//! `IoFidelity::Lossy` rather than claiming a working conversion. See `📓️w4-sourcing-report.md`
//! `## openQuestions`.
use crate::artifacts::curation::schema::snapshot::CurationSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::png::{PngSnapshot, STDIO_PNG_DOCUMENT_SCHEMA};

pub const PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId::ANY };

pub fn deserialize(from: &PngSnapshot) -> Result<CurationSnapshot, store::TextError> {
    let _ = STDIO_PNG_DOCUMENT_SCHEMA;
    let bytes = <PngSnapshot as store::ArtifactPack>::encode_pack(from);
    deserialize_bytes(&bytes)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<CurationSnapshot, store::TextError> {
    <CurationSnapshot as store::ArtifactPack>::decode_pack(bytes).or_else(|_| <CurationSnapshot as store::ArtifactDsl>::parse_dsl(&String::from_utf8_lossy(bytes)))
}

pub struct PngIntoCuration;

impl Deserializer<CurationSnapshot> for PngIntoCuration {
    const FROM: Dialect = PNG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn deserialize(payload: &IoPayload) -> IoResult<CurationSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "PngIntoCuration: expected a binary png payload".to_string(), diagnostics: Vec::new() });
        };
        deserialize_bytes(bytes).map(IoOutcome::clean).map_err(|error| IoError { message: format!("PngIntoCuration: {error}"), diagnostics: Vec::new() })
    }
}
