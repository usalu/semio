//! 🚪️ draw <- dwg — foreign `Deserializer<DrawSnapshot>` (design.md §3).
//!
//! 🕳️ stdio_gap: `s.stdio.semio/v1/drawing` bridges only to svg/dxf/pdf (dwg lives under
//! `s.stdio.semio/v1/cad`, standard `ac1024` — a different hub entirely), so this leaf cannot
//! decode real DWG bytes without hand-rolling DWG parsing again (banned by this ticket). Honest
//! degenerate stub, unchanged behavior from the pre-migration free function this replaces, until
//! stdio grows a drawing<->dwg bridge — see `w5b-w-report.md` `stdio_gaps`.

use crate::artifacts::draw::schema::{create_draw_id, empty_draw_snapshot};
use crate::artifacts::draw::DrawSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const DWG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId::ANY };

pub struct DwgIntoDraw;

impl Deserializer<DrawSnapshot> for DwgIntoDraw {
    const FROM: Dialect = DWG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn deserialize(_payload: &IoPayload) -> IoResult<DrawSnapshot> {
        let mut snap = empty_draw_snapshot();
        snap.id = create_draw_id("dwg-import", b"dwg");
        snap.title = Some("Imported dwg".into());
        Ok(IoOutcome::clean(snap))
    }
}
