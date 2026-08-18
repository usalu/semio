//! 🚪️ curate <- obj — foreign `Deserializer<CurateSnapshot>` (ticket 26/08/17/CLEAN-ARTIFACT-
//! STANDARD-SUBSET-MECHANISM design.md §3).
//!
//! ⚠️ Pre-existing, not fixed this pass: same format-mismatch shape as the sibling `png`/`stl`
//! leaves — `deserialize_bytes` decodes incoming bytes directly as a `CurateSnapshot` pack rather
//! than as an `ObjSnapshot` pack. No domain-correct "OBJ mesh -> curate catalogue" mapping is
//! defined anywhere in this codebase, so both functions are preserved byte-for-byte and honestly
//! labeled `IoFidelity::Lossy`. See `📓️w4-sourcing-report.md` `## openQuestions`.
use crate::artifacts::curate::schema::snapshot::CurateSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::obj::{ObjSnapshot, STDIO_OBJ_DOCUMENT_SCHEMA};

pub const OBJ_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId::ANY };

pub fn deserialize(from: &ObjSnapshot) -> Result<CurateSnapshot, store::TextError> {
    let _ = STDIO_OBJ_DOCUMENT_SCHEMA;
    let bytes = <ObjSnapshot as store::ArtifactPack>::encode_pack(from);
    deserialize_bytes(&bytes)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<CurateSnapshot, store::TextError> {
    <CurateSnapshot as store::ArtifactPack>::decode_pack(bytes).or_else(|_| {
        <CurateSnapshot as store::ArtifactDsl>::parse_dsl(&String::from_utf8_lossy(bytes))
    })
}

pub struct ObjIntoCurate;

impl Deserializer<CurateSnapshot> for ObjIntoCurate {
    const FROM: Dialect = OBJ_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn deserialize(payload: &IoPayload) -> IoResult<CurateSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "ObjIntoCurate: expected a binary obj payload".to_string(), diagnostics: Vec::new() });
        };
        deserialize_bytes(bytes).map(IoOutcome::clean).map_err(|error| IoError { message: format!("ObjIntoCurate: {error}"), diagnostics: Vec::new() })
    }
}
