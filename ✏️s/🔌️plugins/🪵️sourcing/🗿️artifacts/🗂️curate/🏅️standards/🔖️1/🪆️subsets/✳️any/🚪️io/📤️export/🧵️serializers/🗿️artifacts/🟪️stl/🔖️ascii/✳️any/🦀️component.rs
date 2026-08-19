//! 🚪️ curate -> stl — foreign `Serializer<CurateSnapshot>` (ticket 26/08/17/CLEAN-ARTIFACT-
//! STANDARD-SUBSET-MECHANISM design.md §3). See the sibling `Deserializer`'s doc comment: this
//! direction is symmetrically non-functional (format mismatch), preserved byte-for-byte and
//! labeled `IoFidelity::Lossy` honestly.
use crate::artifacts::curate::schema::snapshot::CurateSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::stl::{StlSnapshot, STDIO_STL_DOCUMENT_SCHEMA};

pub const STL_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId::ANY };

pub async fn serialize(snapshot: &CurateSnapshot) -> Result<StlSnapshot, store::TextError> {
    let _ = STDIO_STL_DOCUMENT_SCHEMA;
    let bytes = <CurateSnapshot as store::ArtifactPack>::encode_pack(snapshot);
    <StlSnapshot as store::ArtifactPack>::decode_pack(&bytes)
        .map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}

pub async fn serialize_bytes(snapshot: &CurateSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<StlSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}

pub struct CurateIntoStl;

impl Serializer<CurateSnapshot> for CurateIntoStl {
    const INTO: Dialect = STL_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn serialize(from: &CurateSnapshot) -> IoResult<IoPayload> {
        serialize_bytes(from).map(|bytes| IoOutcome::clean(IoPayload::Binary(bytes))).map_err(|error| IoError { message: format!("CurateIntoStl: {error}"), diagnostics: Vec::new() })
    }
}
