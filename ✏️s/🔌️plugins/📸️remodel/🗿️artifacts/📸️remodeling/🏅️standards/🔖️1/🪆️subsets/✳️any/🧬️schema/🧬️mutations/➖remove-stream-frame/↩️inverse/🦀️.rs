//! ↩️ Inverse for `RemoveStreamFrame` — re-`add-stream-frame`s the captured BASE frame (with the
//! stream's BASE `kind`). Missing target/index ⇒ `Vec::new()`.
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RemoveStreamFrame, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    let Some(stream) = base.streams.iter().find(|stream| stream.id == payload.id) else {
        return Vec::new();
    };
    let Some(frame) = stream.frames.get(payload.frame_index as usize) else {
        return Vec::new();
    };
    vec![crate::artifacts::remodeling::mutations::add_stream_frame::add_stream_frame(payload.id.clone(), frame.clone(), stream.kind)]
}
//#endregion 🔖️Inverse
