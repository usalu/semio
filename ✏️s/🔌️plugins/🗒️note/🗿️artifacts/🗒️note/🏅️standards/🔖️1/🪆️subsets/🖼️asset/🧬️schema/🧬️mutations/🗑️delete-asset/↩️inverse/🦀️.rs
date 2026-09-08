//! ↩️ Inverse for `DeleteAsset`.
use super::DeleteAsset;
use crate::schema::mutations::CreateAsset;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteAsset, base: &NoteSnapshot) -> Vec<NoteMutation> {
    match base.assets.get(&payload.key) {
        Some(prior) => vec![NoteMutation::CreateAsset(CreateAsset { key: payload.key.clone(), asset: prior.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
