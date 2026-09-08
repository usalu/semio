//! 🚪️ note -> json — foreign `Serializer<NoteSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). `NoteSnapshot` round-trips
//! fully through the owned JSON codec — every field survives both directions — so this hop is
//! `IoFidelity::Exact`, matching the sequence pilot's identical json-bridge precedent
//! (`📓️w4-sequence-report.md`). JSON's own native form is text, never a raw-bytes wrapper.

use crate::NoteSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub struct NoteIntoJson;

impl Serializer<NoteSnapshot> for NoteIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &NoteSnapshot) -> IoResult<IoPayload> {
        Ok(IoOutcome::clean(IoPayload::Text(dsl::os_pack::json::to_string_pretty(&dsl::os_pack::json_from_dsl_value(&dsl::ToValue::to_value(from))))))
    }
}
