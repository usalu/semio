//! 🚪️ block3d → json — foreign `Serializer<Block3dSnapshot>` on the framework's `io_mechanism`
//! channel. The snapshot is a pure `dsl::ToValue` record tree, so its rfc8259 rendition carries every
//! field and the sibling `📥️import` leaf reconstructs the snapshot exactly: `IoFidelity::Exact`.

use crate::artifacts::block3d::Block3dSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::write_json_text;
use semio_s_plugin_stdio::artifacts::json::JsonSnapshot;

/// 🎯️ The foreign dialect this leaf writes.
pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

/// 🔣️ This subset's snapshot as compact rfc8259 text — also the body the `🎒️zip` container leaf
/// embeds, and the exact bytes the TypeScript mirror's parity test compares against.
pub fn json_text(from: &Block3dSnapshot) -> String {
    write_json_text(&JsonSnapshot::from_value(dsl::json::from_dsl_value(&dsl::ToValue::to_value(from))).value)
}

/// 🧵️ `s.block.block3d@1/*` → `s.stdio.json@rfc8259/*`.
pub struct Block3dIntoJson;

impl Serializer<Block3dSnapshot> for Block3dIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &Block3dSnapshot) -> IoResult<IoPayload> {
        Ok(IoOutcome::clean(IoPayload::Text(json_text(from))))
    }
}
