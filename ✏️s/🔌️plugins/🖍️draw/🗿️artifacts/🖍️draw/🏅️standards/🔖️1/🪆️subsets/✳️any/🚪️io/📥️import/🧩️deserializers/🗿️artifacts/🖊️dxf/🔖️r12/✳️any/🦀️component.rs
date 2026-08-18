//! 🚪️ draw <- dxf — foreign `Deserializer<DrawSnapshot>` (design.md §3). Honest not-yet-
//! implemented stub (unchanged behavior from the pre-migration free function this replaces): real
//! DXF parsing into draw layers is out of scope for this cutover. `IoFidelity::Lossy`.

use crate::artifacts::draw::schema::{create_draw_id, empty_draw_snapshot};
use crate::artifacts::draw::DrawSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const DXF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dxf", standard: StandardId("r12"), subset: SubsetId::ANY };

pub struct DxfIntoDraw;

impl Deserializer<DrawSnapshot> for DxfIntoDraw {
    const FROM: Dialect = DXF_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn deserialize(_payload: &IoPayload) -> IoResult<DrawSnapshot> {
        let mut snap = empty_draw_snapshot();
        snap.id = create_draw_id("dxf-import", b"dxf");
        snap.title = Some("Imported dxf".into());
        Ok(IoOutcome::clean(snap))
    }
}
