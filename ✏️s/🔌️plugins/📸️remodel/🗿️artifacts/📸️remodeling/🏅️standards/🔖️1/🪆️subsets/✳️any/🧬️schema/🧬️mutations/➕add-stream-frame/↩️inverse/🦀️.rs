//! ↩️ Inverse for `AddStreamFrame` — `remove-stream-frame` targeting the canonical `(index, asset_id)`
//! position the frame is inserted at, computed from BASE the same way the diff computes it. A step the
//! forward verb refuses or warns off (unknown stream, declared media kind the stream does not have, a
//! frame the stream already holds) moved nothing, so its inverse is `Vec::new()`.
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::AddStreamFrame, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    let Some(stream) = base.streams.iter().find(|stream| stream.id == payload.id) else {
        return Vec::new();
    };
    if payload.kind != stream.kind || stream.frames.contains(&payload.frame) {
        return Vec::new();
    }
    let at = crate::artifacts::remodeling::mutations::ordered_index(&stream.frames, &(payload.frame.index, payload.frame.asset_id.clone()), |frame| (frame.index, frame.asset_id.clone()));
    vec![crate::artifacts::remodeling::mutations::remove_stream_frame::remove_stream_frame(payload.id.clone(), at as u32)]
}
//#endregion 🔖️Inverse
