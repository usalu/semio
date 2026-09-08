//! ↩️ Inverse for `ChangeStreamSync` — the OLD `sync_offset_ms` looked up from BASE.
//! Missing target ⇒ `Vec::new()`.
use crate::mutations::RemodelingMutation;
use crate::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ChangeStreamSync, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    match base.streams.iter().find(|stream| stream.id == payload.id) {
        Some(stream) => vec![super::change_stream_sync(payload.id.clone(), stream.sync_offset_ms)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
