//! draw <- dwg
//!
//! 🕳️ stdio_gap: `s.stdio.semio/v1/drawing` bridges only to svg/dxf/pdf (dwg lives under
//! `s.stdio.semio/v1/cad`, standard `ac1024` — a different hub entirely), so this leaf can no
//! longer decode real DWG bytes without hand-rolling DWG parsing again (banned by this ticket).
//! Honest degenerate stub, same shape as this subset's svg/pdf/png import siblings, until stdio
//! grows a drawing↔dwg bridge — see `w5b-w-report.md` `stdio_gaps`.
use crate::artifacts::draw::engine::{create_draw_id, empty_draw_snapshot};
use crate::artifacts::draw::DrawSnapshot;
pub fn register() {}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<DrawSnapshot, String> {
    let _ = bytes;
    let mut snap = empty_draw_snapshot();
    snap.id = create_draw_id("dwg-import", b"dwg");
    snap.title = Some("Imported dwg".into());
    Ok(snap)
}
