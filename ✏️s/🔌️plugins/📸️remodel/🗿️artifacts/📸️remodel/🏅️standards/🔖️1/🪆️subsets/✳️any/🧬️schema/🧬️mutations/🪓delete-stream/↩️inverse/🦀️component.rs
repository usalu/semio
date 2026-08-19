//! ↩️ Inverse for `DeleteStream` — reconstructs a `create-stream` of the captured BASE stream.
//! Missing target ⇒ `Vec::new()`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::DeleteStream, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    match base.streams.iter().find(|stream| stream.id == payload.id) {
        Some(stream) => vec![crate::artifacts::remodel::mutations::create_stream::mutation::create_stream(stream.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
