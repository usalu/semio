//! ↩️ Inverse for `DeleteAsset`.
use super::mutation::DeleteAsset;
use crate::artifacts::note::schema::mutations::CreateAsset;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &DeleteAsset, base: &NoteSnapshot) -> Vec<NoteMutation> {
    match base.assets.get(&payload.key) {
        Some(prior) => vec![NoteMutation::CreateAsset(CreateAsset { key: payload.key.clone(), asset: prior.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
