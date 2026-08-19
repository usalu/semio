//! ↩️ Inverse for `RemoveStreamFrame` — re-`add-stream-frame`s the captured BASE frame (with the
//! stream's BASE `kind`). Missing target/index ⇒ `Vec::new()`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::RemoveStreamFrame, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    let Some(stream) = base.streams.iter().find(|stream| stream.id == payload.id) else {
        return Vec::new();
    };
    let Some(frame) = stream.frames.get(payload.frame_index as usize) else {
        return Vec::new();
    };
    vec![crate::artifacts::remodel::mutations::add_stream_frame::mutation::add_stream_frame(payload.id.clone(), frame.clone(), stream.kind)]
}
//#endregion 🔖️Inverse
