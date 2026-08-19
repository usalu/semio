//! 🚪️ dag <- json — foreign `Deserializer<DagSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Every `DagSnapshot` field
//! round-trips through `serde_json` untouched, so this hop is `IoFidelity::Exact`.
//!
//! 🐛️ Replaces a pre-migration bug: the old `deserialize_text` parsed the incoming text as this
//! plugin's OWN `.dag` DSL directly, never as real json — this impl decodes the foreign
//! `JsonSnapshot` first, via its own `ArtifactPack`, as the coordinate (`JSON_DIALECT`) requires.

use crate::artifacts::dag::DagSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

/// 📖️ Typed decode of a `JsonSnapshot`'s free-form `value` into `DagSnapshot`'s own field shape.
pub async fn deserialize(from: &JsonSnapshot) -> Result<DagSnapshot, store::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    serde_json::from_value(from.to_serde_value()).map_err(|e| store::TextError::new(format!("dag<-json: {e}"), dsl::TextSpan::at(1, 1)))
}

pub struct JsonIntoDag;

impl Deserializer<DagSnapshot> for JsonIntoDag {
    const FROM: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn deserialize(payload: &IoPayload) -> IoResult<DagSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "JsonIntoDag: expected a binary json payload".to_string(), diagnostics: Vec::new() });
        };
        let json = <JsonSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| IoError { message: format!("JsonIntoDag: json decode failed: {error}"), diagnostics: Vec::new() })?;
        let snapshot = deserialize(&json).map_err(|error| IoError { message: format!("JsonIntoDag: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(snapshot))
    }
}
