//! ↩️ Inverse for `AddStreamFrame` — `remove-stream-frame` targeting the index the appended frame
//! will land at (BASE's frame count). Missing target ⇒ `Vec::new()`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::AddStreamFrame, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    match base.streams.iter().find(|stream| stream.id == payload.id) {
        Some(stream) => vec![crate::artifacts::remodel::mutations::remove_stream_frame::remove_stream_frame(payload.id.clone(), stream.frames.len() as u32)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
