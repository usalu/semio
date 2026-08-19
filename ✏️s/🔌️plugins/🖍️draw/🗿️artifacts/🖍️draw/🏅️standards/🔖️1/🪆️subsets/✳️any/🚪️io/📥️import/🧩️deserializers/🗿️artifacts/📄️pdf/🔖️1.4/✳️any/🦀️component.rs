//! 🚪️ draw <- pdf — foreign `Deserializer<DrawSnapshot>` (design.md §3). Honest not-yet-
//! implemented stub (unchanged behavior from the pre-migration free function this replaces): real
//! PDF parsing into draw layers is out of scope for this cutover. `IoFidelity::Lossy`.

use crate::artifacts::draw::schema::{create_draw_id, empty_draw_snapshot};
use crate::artifacts::draw::DrawSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const PDF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId::ANY };

pub struct PdfIntoDraw;

impl Deserializer<DrawSnapshot> for PdfIntoDraw {
    const FROM: Dialect = PDF_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn deserialize(_payload: &IoPayload) -> IoResult<DrawSnapshot> {
        let mut snap = empty_draw_snapshot();
        snap.id = create_draw_id("pdf-import", b"pdf");
        snap.title = Some("Imported pdf".into());
        Ok(IoOutcome::clean(snap))
    }
}
