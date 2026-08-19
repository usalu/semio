//! ↩️ Inverse for `ReplaceStreamSource` — the OLD `source` looked up from BASE.
//! Missing target ⇒ `Vec::new()`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::ReplaceStreamSource, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    match base.streams.iter().find(|stream| stream.id == payload.id) {
        Some(stream) => vec![super::mutation::replace_stream_source(payload.id.clone(), stream.source.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
