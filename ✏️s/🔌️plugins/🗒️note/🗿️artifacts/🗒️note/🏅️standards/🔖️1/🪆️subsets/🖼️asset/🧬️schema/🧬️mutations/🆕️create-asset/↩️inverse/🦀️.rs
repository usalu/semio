//! ↩️ Inverse for `CreateAsset`.
use super::CreateAsset;
use crate::artifacts::note::schema::mutations::DeleteAsset;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &CreateAsset, _base: &NoteSnapshot) -> Vec<NoteMutation> {
    vec![NoteMutation::DeleteAsset(DeleteAsset { key: payload.key.clone() })]
}
//#endregion 🔖️Inverse
