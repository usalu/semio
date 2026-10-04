//! 🚪️ block2d → json — foreign `Serializer<Block2dSnapshot>` on the framework's `io_mechanism`
//! channel. Named binary64 fields use closed word objects and retain every IEEE state.

use crate::Block2dSnapshot;
use semio_framework::io::io_mechanism::{ArchiveChildren, Serializer};
use semio_framework::io_schema::{Dialect, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

/// 🎯️ The foreign dialect this leaf writes.
pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

/// 🔣️ This subset's snapshot as compact rfc8259 text — also the body the `🎒️zip` container leaf
/// embeds, and the exact bytes the TypeScript mirror's parity test compares against.
pub fn json_text(from: &Block2dSnapshot) -> String {
    crate::standards::v1::subsets::any::io::json::to_json_text(from)
}

/// 🧵️ `s.block.block2d@1/*` → `s.stdio.json@rfc8259/*`.
pub struct Block2dIntoJson;

impl Serializer<Block2dSnapshot> for Block2dIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &Block2dSnapshot, _: &ArchiveChildren) -> IoResult<IoPayload> {
        Ok(IoOutcome::clean(IoPayload::Text(json_text(from))))
    }
}
