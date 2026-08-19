//! 🚪️ curate -> json — foreign `Serializer<CurateSnapshot>` (ticket 26/08/17/CLEAN-ARTIFACT-
//! STANDARD-SUBSET-MECHANISM design.md §3). Symmetric with the sibling `Deserializer`: a genuine
//! `serde_json` structural round trip, `IoFidelity::Exact`. Bridges via json's own RFC8259 text
//! codec (`write_json_pretty`), matching `s/plugin/lowpoly`'s identical export leaf.
use crate::artifacts::curate::CurateSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::write_json_pretty;
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub async fn serialize(snapshot: &CurateSnapshot) -> Result<JsonSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    let value = serde_json::to_value(snapshot).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    Ok(JsonSnapshot::from_value(value))
}

pub async fn serialize_bytes(snapshot: &CurateSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(write_json_pretty(&serialize(snapshot)?.value).into_bytes())
}

pub struct CurateIntoJson;

impl Serializer<CurateSnapshot> for CurateIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &CurateSnapshot) -> IoResult<IoPayload> {
        let bytes = serialize_bytes(from).map_err(|error| IoError { message: format!("CurateIntoJson: {error}"), diagnostics: Vec::new() })?;
        let text = String::from_utf8(bytes).map_err(|error| IoError { message: format!("CurateIntoJson: non-utf8 json output: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(IoPayload::Text(text)))
    }
}
