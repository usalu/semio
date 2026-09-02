//! 🚪️ draw -> dwg — foreign `Serializer<DrawSnapshot>` (design.md §3). Honest not-yet-implemented
//! stub: the pre-migration free function this replaces printed the artifact's OWN `.draw` DSL text
//! and mislabeled it as DWG bytes — a real correctness bug. Fixed here by refusing honestly instead
//! of perpetuating the mislabeled payload. See the sibling import leaf's `stdio_gap` note: stdio
//! has no drawing<->dwg bridge yet, so real DWG export is out of scope for this cutover regardless.

use crate::artifacts::draw::DrawSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const DWG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId::ANY };

pub struct DrawIntoDwg;

impl Serializer<DrawSnapshot> for DrawIntoDwg {
    const INTO: Dialect = DWG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn serialize(_from: &DrawSnapshot) -> IoResult<IoPayload> {
        Err(IoError { message: "DrawIntoDwg: DWG export is not yet implemented (no stdio drawing<->dwg bridge)".to_string(), diagnostics: Vec::new() })
    }
}
