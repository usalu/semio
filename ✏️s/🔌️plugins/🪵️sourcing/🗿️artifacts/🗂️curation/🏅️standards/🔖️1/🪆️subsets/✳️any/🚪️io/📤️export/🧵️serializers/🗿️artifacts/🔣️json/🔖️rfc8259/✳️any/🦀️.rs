//! 🚪️ curation -> json — foreign `Serializer<CurationSnapshot>` (ticket 26/08/17/CLEAN-ARTIFACT-
//! STANDARD-SUBSET-MECHANISM design.md §3). Symmetric with the sibling `Deserializer`: a genuine
//! `serde_json` structural round trip, `IoFidelity::Exact`. Bridges via json's own RFC8259 text
//! codec (`write_json_pretty`), matching `s/plugin/lowpoly`'s identical export leaf.
use crate::CurationSnapshot;
use semio_framework_value::ToValue;
use semio_framework_os_kernel::io::io_mechanism::{ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::io::text::snapshot::write_json_pretty;
use semio_s_artifact_stdio_json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub fn serialize(snapshot: &CurationSnapshot) -> Result<JsonSnapshot, semio_framework_diagnostic::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    Ok(JsonSnapshot::from_value(semio_framework_pack_json::from_dsl_value(&snapshot.to_value())))
}

pub fn serialize_bytes(snapshot: &CurationSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    Ok(write_json_pretty(&serialize(snapshot)?.value).into_bytes())
}

pub struct CurationIntoJson;

impl Serializer<CurationSnapshot> for CurationIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &CurationSnapshot, _: &ArchiveChildren) -> IoResult<IoPayload> {
        let bytes = serialize_bytes(from).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("CurationIntoJson: {error}"))))?;
        let text = String::from_utf8(bytes).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("CurationIntoJson: non-utf8 json output: {error}"))))?;
        Ok(IoOutcome::clean(IoPayload::Text(text)))
    }
}
