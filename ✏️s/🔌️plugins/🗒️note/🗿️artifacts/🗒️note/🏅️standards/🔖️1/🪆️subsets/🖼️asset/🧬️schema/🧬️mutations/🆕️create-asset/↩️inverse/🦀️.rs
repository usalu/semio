//! ↩️ Inverse for `CreateAsset`.
use super::CreateAsset;
use crate::schema::mutations::DeleteAsset;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateAsset, _base: &NoteSnapshot) -> Vec<NoteMutation> {
    vec![NoteMutation::DeleteAsset(DeleteAsset { key: payload.key.clone() })]
}
//#endregion 🔖️Inverse
