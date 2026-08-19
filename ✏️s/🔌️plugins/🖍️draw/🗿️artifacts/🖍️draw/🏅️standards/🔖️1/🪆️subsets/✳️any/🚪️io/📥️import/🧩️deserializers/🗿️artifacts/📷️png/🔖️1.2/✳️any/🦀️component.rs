//! 🚪️ draw <- png — foreign `Deserializer<DrawSnapshot>` (design.md §3). Honest not-yet-
//! implemented stub (unchanged behavior from the pre-migration free function this replaces): real
//! raster tracing into draw layers is out of scope for this cutover. `IoFidelity::Lossy`.

use crate::artifacts::draw::schema::{create_draw_id, empty_draw_snapshot};
use crate::artifacts::draw::DrawSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId::ANY };

pub struct PngIntoDraw;

impl Deserializer<DrawSnapshot> for PngIntoDraw {
    const FROM: Dialect = PNG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn deserialize(_payload: &IoPayload) -> IoResult<DrawSnapshot> {
        let mut snap = empty_draw_snapshot();
        snap.id = create_draw_id("png-import", b"png");
        snap.title = Some("Imported png".into());
        Ok(IoOutcome::clean(snap))
    }
}
