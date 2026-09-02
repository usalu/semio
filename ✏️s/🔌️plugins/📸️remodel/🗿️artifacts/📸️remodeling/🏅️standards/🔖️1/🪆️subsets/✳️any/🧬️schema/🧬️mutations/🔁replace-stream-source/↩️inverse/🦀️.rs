//! ↩️ Inverse for `ReplaceStreamSource` — the OLD `source` looked up from BASE.
//! Missing target ⇒ `Vec::new()`.
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ReplaceStreamSource, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    match base.streams.iter().find(|stream| stream.id == payload.id) {
        Some(stream) => vec![super::replace_stream_source(payload.id.clone(), stream.source.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
