//! 🚪️ drawing <- dxf — foreign `Deserializer<DrawingSnapshot>` (design.md §3). Honest not-yet-
//! implemented stub (unchanged behavior from the pre-migration free function this replaces): real
//! DXF parsing into drawing layers is out of scope for this cutover. `IoFidelity::Lossy`.

use crate::artifacts::drawing::schema::{create_drawing_id, empty_drawing_snapshot};
use crate::artifacts::drawing::DrawingSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const DXF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dxf", standard: StandardId("r12"), subset: SubsetId::ANY };

pub struct DxfIntoDraw;

impl Deserializer<DrawingSnapshot> for DxfIntoDraw {
    const FROM: Dialect = DXF_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn deserialize(_payload: &IoPayload) -> IoResult<DrawingSnapshot> {
        let mut snap = empty_drawing_snapshot();
        snap.id = create_drawing_id("dxf-import", b"dxf");
        snap.title = Some("Imported dxf".into());
        Ok(IoOutcome::clean(snap))
    }
}
