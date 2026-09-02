//! 🚪️ drawing <- svg — foreign `Deserializer<DrawingSnapshot>` (design.md §3). Honest not-yet-
//! implemented stub (unchanged behavior from the pre-migration free function this replaces): real
//! SVG parsing into drawing layers is out of scope for this cutover, so this always returns a
//! placeholder empty document rather than fabricating shapes. `IoFidelity::Lossy` — real content is
//! never actually recovered.

use crate::artifacts::drawing::schema::{create_drawing_id, empty_drawing_snapshot};
use crate::artifacts::drawing::DrawingSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId::ANY };

pub struct SvgIntoDraw;

impl Deserializer<DrawingSnapshot> for SvgIntoDraw {
    const FROM: Dialect = SVG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn deserialize(_payload: &IoPayload) -> IoResult<DrawingSnapshot> {
        let mut snap = empty_drawing_snapshot();
        snap.id = create_drawing_id("svg-import", b"svg");
        snap.title = Some("Imported svg".into());
        Ok(IoOutcome::clean(snap))
    }
}
