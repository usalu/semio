//! 🚪️ drawing <- png — foreign `Deserializer<DrawingSnapshot>` (design.md §3). Honest not-yet-
//! implemented stub (unchanged behavior from the pre-migration free function this replaces): real
//! raster tracing into drawing layers is out of scope for this cutover. `IoFidelity::Lossy`.

use crate::artifacts::drawing::schema::{create_drawing_id, empty_drawing_snapshot};
use crate::artifacts::drawing::DrawingSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId::ANY };

pub struct PngIntoDraw;

impl Deserializer<DrawingSnapshot> for PngIntoDraw {
    const FROM: Dialect = PNG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn deserialize(_payload: &IoPayload) -> IoResult<DrawingSnapshot> {
        let mut snap = empty_drawing_snapshot();
        snap.id = create_drawing_id("png-import", b"png");
        snap.title = Some("Imported png".into());
        Ok(IoOutcome::clean(snap))
    }
}
