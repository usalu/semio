//! 🚪️ block3d → json — foreign `Serializer<Block3dSnapshot>` on the framework's `io_mechanism`
//! channel. The snapshot is a pure `dsl::ToValue` record tree, so its rfc8259 rendition carries every
//! field and the sibling `📥️import` leaf reconstructs the snapshot exactly: `IoFidelity::Exact`.

use crate::Block3dSnapshot;
use semio_framework_os_kernel::io::io_mechanism::{ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::io::text::snapshot::write_json_text;
use semio_s_artifact_stdio_json::JsonSnapshot;

/// 🎯️ The foreign dialect this leaf writes.
pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

/// 🔣️ This subset's snapshot as compact rfc8259 text — also the body the `🎒️zip` container leaf
/// embeds, and the exact bytes the TypeScript mirror's parity test compares against.
pub fn json_text(from: &Block3dSnapshot) -> String {
    let mut value = semio_framework_value::ToValue::to_value(from);
    crate::standards::v1::subsets::any::schema::snapshot::json_transport::convert(&mut value, false).expect("typed Block3d floating roles");
    write_json_text(&JsonSnapshot::from_value(semio_framework_pack_json::from_dsl_value(&value)).value)
}

/// 🧵️ `s.block.block3d@1/*` → `s.stdio.json@rfc8259/*`.
pub struct Block3dIntoJson;

impl Serializer<Block3dSnapshot> for Block3dIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &Block3dSnapshot, _: &ArchiveChildren, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        Ok(IoOutcome::clean(IoPayload::Text(json_text(from))))
    }
}
