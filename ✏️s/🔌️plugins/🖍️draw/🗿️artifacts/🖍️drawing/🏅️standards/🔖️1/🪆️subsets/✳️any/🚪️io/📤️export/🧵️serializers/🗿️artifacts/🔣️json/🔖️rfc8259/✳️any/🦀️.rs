//! 🚪️ drawing -> json — foreign `Serializer<DrawingSnapshot>` (design.md §3). Real: bridges via
//! stdio's own real RFC8259 text writer (`write_json_pretty`), not `serde_json::to_string`, since
//! `JsonSnapshot.value` is stdio's own lexeme-preserving `JsonValue` model. `IoFidelity::Exact`.
//! Goes through `semio_framework_value::ToValue`/`semio_framework_pack_json::from_dsl_value` — no `serde_json` anywhere in this file.

use crate::DrawingSnapshot;
use semio_framework::io::io_mechanism::{ArchiveChildren, Serializer};
use semio_framework::io_schema::{Dialect, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::io::text::snapshot::write_json_pretty;
use semio_s_artifact_stdio_json::JsonSnapshot;

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub struct DrawingIntoJson;

impl Serializer<DrawingSnapshot> for DrawingIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &DrawingSnapshot, _: &ArchiveChildren) -> IoResult<IoPayload> {
        let value = semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(from));
        let json = JsonSnapshot::from_value(value);
        Ok(IoOutcome::clean(IoPayload::Text(write_json_pretty(&json.value))))
    }
}
