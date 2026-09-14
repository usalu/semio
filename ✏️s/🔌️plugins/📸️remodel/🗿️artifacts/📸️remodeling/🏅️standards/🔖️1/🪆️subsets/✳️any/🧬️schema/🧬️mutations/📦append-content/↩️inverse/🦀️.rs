//! ↩️ Inverse for `AppendContent` — remove every leaf past the leaf count the BASE stored (0 removes
//! the entry); an append that adds nothing inverts to nothing.
use crate::mutations::{remove_content, RemodelingMutation};
use crate::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::AppendContent, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    let stored = base.durable_artifacts.get(&payload.content_id).map_or(0, |artifact| artifact.chunks.len() as u64);
    if payload.first + payload.chunks.len() as u64 <= stored {
        return Vec::new();
    }
    vec![remove_content(payload.content_id.clone(), stored)]
}
//#endregion 🔖️Inverse
