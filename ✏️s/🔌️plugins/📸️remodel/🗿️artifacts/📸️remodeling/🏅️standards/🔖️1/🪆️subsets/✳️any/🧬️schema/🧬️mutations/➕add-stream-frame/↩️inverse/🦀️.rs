//! ↩️ Inverse for `AddStreamFrame` — `remove-stream-frame` targeting the index the appended frame
//! will land at (BASE's frame count). Missing target ⇒ `Vec::new()`.
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::AddStreamFrame, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    match base.streams.iter().find(|stream| stream.id == payload.id) {
        Some(stream) => vec![crate::artifacts::remodeling::mutations::remove_stream_frame::remove_stream_frame(payload.id.clone(), stream.frames.len() as u32)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
