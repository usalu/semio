//! ↩️ Inverse for `RemoveStreamFrame` — re-`add-stream-frame`s the captured BASE frame, which lands
//! at its canonical `(index, asset_id)` position, i.e. exactly the one it was removed from. `kind` is
//! the stream's own BASE kind, which is what the forward verb now asserts rather than rewrites.
//! Missing target/index ⇒ `Vec::new()`.
use crate::mutations::RemodelingMutation;
use crate::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RemoveStreamFrame, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    let Some(stream) = base.streams.iter().find(|stream| stream.id == payload.id) else {
        return Vec::new();
    };
    let Some(frame) = stream.frames.get(payload.frame_index as usize) else {
        return Vec::new();
    };
    vec![crate::mutations::add_stream_frame::add_stream_frame(payload.id.clone(), frame.clone(), stream.kind)]
}
//#endregion 🔖️Inverse
