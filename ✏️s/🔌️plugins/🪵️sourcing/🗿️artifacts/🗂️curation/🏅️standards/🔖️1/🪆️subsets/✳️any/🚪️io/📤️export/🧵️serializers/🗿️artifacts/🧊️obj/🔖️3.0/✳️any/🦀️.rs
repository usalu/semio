//! 🚪️ curation -> obj — foreign `Serializer<CurationSnapshot>` (ticket 26/08/17/CLEAN-ARTIFACT-
//! STANDARD-SUBSET-MECHANISM design.md §3). See the sibling `Deserializer`'s doc comment: this
//! direction is symmetrically non-functional (format mismatch), preserved byte-for-byte and
//! labeled `IoFidelity::Lossy` honestly.
use crate::artifacts::curation::schema::snapshot::CurationSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::obj::{ObjSnapshot, STDIO_OBJ_DOCUMENT_SCHEMA};

pub const OBJ_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId::ANY };

pub fn serialize(snapshot: &CurationSnapshot) -> Result<ObjSnapshot, store::TextError> {
    let _ = STDIO_OBJ_DOCUMENT_SCHEMA;
    let bytes = <CurationSnapshot as store::ArtifactPack>::encode_pack(snapshot);
    <ObjSnapshot as store::ArtifactPack>::decode_pack(&bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
}

pub fn serialize_bytes(snapshot: &CurationSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<ObjSnapshot as store::ArtifactPack>::encode_pack(&serialize(snapshot)?))
}

pub struct CurationIntoObj;

impl Serializer<CurationSnapshot> for CurationIntoObj {
    const INTO: Dialect = OBJ_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &CurationSnapshot) -> IoResult<IoPayload> {
        serialize_bytes(from).map(|bytes| IoOutcome::clean(IoPayload::Binary(bytes))).map_err(|error| IoError { message: format!("CurationIntoObj: {error}"), diagnostics: Vec::new() })
    }
}
