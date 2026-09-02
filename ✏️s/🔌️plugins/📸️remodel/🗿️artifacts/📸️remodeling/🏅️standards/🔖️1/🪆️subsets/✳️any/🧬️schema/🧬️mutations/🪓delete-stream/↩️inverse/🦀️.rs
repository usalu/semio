//! ↩️ Inverse for `DeleteStream` — reconstructs a `create-stream` of the captured BASE stream.
//! Missing target ⇒ `Vec::new()`.
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteStream, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    match base.streams.iter().find(|stream| stream.id == payload.id) {
        Some(stream) => vec![crate::artifacts::remodeling::mutations::create_stream::create_stream(stream.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
