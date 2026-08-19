//! 🚪️ note -> json — foreign `Serializer<NoteSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). `NoteSnapshot` round-trips
//! fully through `serde_json` — every field survives both directions — so this hop is
//! `IoFidelity::Exact`, matching the sequence pilot's identical json-bridge precedent
//! (`📓️w4-sequence-report.md`). JSON's own native form is text, never a raw-bytes wrapper.

use crate::artifacts::note::NoteSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::{write_json_pretty, JsonSnapshot};

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub struct NoteIntoJson;

impl Serializer<NoteSnapshot> for NoteIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &NoteSnapshot) -> IoResult<IoPayload> {
        let value = serde_json::to_value(from).map_err(|error| IoError { message: format!("NoteIntoJson: {error}"), diagnostics: Vec::new() })?;
        let json = JsonSnapshot::from_value(value);
        Ok(IoOutcome::clean(IoPayload::Text(write_json_pretty(&json.value))))
    }
}
