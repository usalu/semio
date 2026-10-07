//! 🚪️ note <- json — foreign `Deserializer<NoteSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). `NoteSnapshot` round-trips
//! fully through the owned JSON codec — every field survives both directions — so this hop is
//! `IoFidelity::Exact`, matching the sibling export leaf and the sequence pilot's identical
//! json-bridge precedent (`📓️w4-sequence-report.md`).

use crate::{NoteSnapshot, NOTE_DOCUMENT_SCHEMA};
use semio_framework::io::io_mechanism::Deserializer;
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub struct JsonIntoNote;

impl Deserializer<NoteSnapshot> for JsonIntoNote {
    const FROM: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn deserialize(payload: &IoPayload) -> IoResult<NoteSnapshot> {
        let IoPayload::Text(text) = payload else {
            return Err(IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "JsonIntoNote: expected a text json payload".to_string())));
        };
        let mut snap: NoteSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("JsonIntoNote: {error}"))))?;
        if snap.schema.is_empty() {
            snap.schema = NOTE_DOCUMENT_SCHEMA.into();
        }
        Ok(IoOutcome::clean(snap))
    }
}
