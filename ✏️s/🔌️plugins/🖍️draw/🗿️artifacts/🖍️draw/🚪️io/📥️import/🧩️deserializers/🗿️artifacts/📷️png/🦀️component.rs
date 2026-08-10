//! draw <- png
use crate::artifacts::draw::engine::{empty_draw_snapshot, create_draw_id};
use crate::artifacts::draw::DrawSnapshot;
pub fn register() {}
pub fn deserialize_bytes(bytes: &[u8]) -> Result<DrawSnapshot, String> {
    let _ = bytes;
    let mut snap = empty_draw_snapshot();
    snap.id = create_draw_id("png-import", b"png");
    snap.title = Some(format!("Imported png"));
    Ok(snap)
}
