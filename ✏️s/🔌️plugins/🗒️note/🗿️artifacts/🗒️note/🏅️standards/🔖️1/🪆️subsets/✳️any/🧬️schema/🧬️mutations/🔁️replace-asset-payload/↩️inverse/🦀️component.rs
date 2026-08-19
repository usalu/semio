//! ↩️ Inverse for `ReplaceAssetPayload`.
use super::mutation::ReplaceAssetPayload;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &ReplaceAssetPayload, base: &NoteSnapshot) -> Vec<NoteMutation> {
    match base.assets.get(&payload.key) {
        Some(prior) => vec![NoteMutation::ReplaceAssetPayload(ReplaceAssetPayload { key: payload.key.clone(), new_asset: prior.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
