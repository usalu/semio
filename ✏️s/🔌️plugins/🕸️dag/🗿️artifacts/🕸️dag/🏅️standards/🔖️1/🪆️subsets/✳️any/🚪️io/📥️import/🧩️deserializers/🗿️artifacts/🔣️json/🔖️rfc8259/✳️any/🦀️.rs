//! 🚪️ dag <- json — foreign `Deserializer<DagSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Every `DagSnapshot` field
//! round-trips through `serde_json` untouched, so this hop is `IoFidelity::Exact`.
//!
//! 🐛️ Replaces a pre-migration bug: the old `deserialize_text` parsed the incoming text as this
//! plugin's OWN `.dag` DSL directly, never as real json — this impl decodes the foreign
//! `JsonSnapshot` first, via its own `ArtifactPack`, as the coordinate (`JSON_DIALECT`) requires.

use crate::DagSnapshot;
use semio_framework_value::FromValue;
use semio_framework_os_kernel::io::io_mechanism::Deserializer;
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_json::{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA};

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

/// 📖️ Typed decode of a `JsonSnapshot`'s free-form `value` into `DagSnapshot`'s own field shape.
pub fn deserialize(from: &JsonSnapshot) -> Result<DagSnapshot, semio_framework_diagnostic::TextError> {
    let _ = STDIO_JSON_DOCUMENT_SCHEMA;
    DagSnapshot::from_value(semio_framework_pack_json::to_dsl_value(&from.to_pack_value())).map_err(|e| semio_framework_diagnostic::TextError::new(e.kind, format!("dag<-json: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1)))
}

pub struct JsonIntoDag;

impl Deserializer<DagSnapshot> for JsonIntoDag {
    const FROM: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn deserialize(payload: &IoPayload, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<DagSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "JsonIntoDag: expected a binary json payload".to_string())));
        };
        let json = <JsonSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("JsonIntoDag: json decode failed: {error}"))))?;
        let snapshot = deserialize(&json).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("JsonIntoDag: {error}"))))?;
        Ok(IoOutcome::clean(snapshot))
    }
}
