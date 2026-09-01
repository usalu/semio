//! ↩️ Inverse for `CreateStream` — `delete-stream` of the id it created. A duplicate create (id
//! already present in `base`) was itself a no-op, so its inverse is also `Vec::new()`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::CreateStream, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    if base.streams.iter().any(|stream| stream.id == payload.stream.id) {
        return Vec::new();
    }
    vec![crate::artifacts::remodel::mutations::delete_stream::delete_stream(payload.stream.id.clone())]
}
//#endregion 🔖️Inverse
