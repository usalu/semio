//! 🚪️ curate -> png — foreign `Serializer<CurateSnapshot>` (ticket 26/08/17/CLEAN-ARTIFACT-
//! STANDARD-SUBSET-MECHANISM design.md §3). See the sibling `Deserializer`'s doc comment: this
//! direction is symmetrically non-functional (encodes `CurateSnapshot`'s own pack, then tries to
//! decode it as a `PngSnapshot` pack — always fails for a format mismatch), preserved byte-for-byte
//! and labeled `IoFidelity::Lossy` honestly rather than claiming a working conversion.
use crate::artifacts::curate::schema::snapshot::CurateSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::png::{PngSnapshot, STDIO_PNG_DOCUMENT_SCHEMA};

pub const PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId::ANY };

pub fn serialize(snapshot: &CurateSnapshot) -> Result<PngSnapshot, store::TextError> {
    let _ = STDIO_PNG_DOCUMENT_SCHEMA;
    let bytes = <CurateSnapshot as store::ArtifactPack>::encode_pack(snapshot);
    <PngSnapshot as store::ArtifactPack>::decode_pack(&bytes)
        .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}

pub fn serialize_bytes(snapshot: &CurateSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<PngSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}

pub struct CurateIntoPng;

impl Serializer<CurateSnapshot> for CurateIntoPng {
    const INTO: Dialect = PNG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn serialize(from: &CurateSnapshot) -> IoResult<IoPayload> {
        serialize_bytes(from).map(|bytes| IoOutcome::clean(IoPayload::Binary(bytes))).map_err(|error| IoError { message: format!("CurateIntoPng: {error}"), diagnostics: Vec::new() })
    }
}
