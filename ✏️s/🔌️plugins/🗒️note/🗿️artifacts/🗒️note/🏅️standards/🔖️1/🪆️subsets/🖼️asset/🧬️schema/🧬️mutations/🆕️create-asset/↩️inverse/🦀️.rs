//! ↩️ Inverse for `CreateAsset`.
use super::CreateAsset;
use crate::schema::mutations::{DeleteAsset, NoteMutation};
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateAsset, base: &NoteSnapshot) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    Ok(match base.assets.contains_key(&payload.key) {
        true => Vec::new(),
        false => vec![NoteMutation::DeleteAsset(DeleteAsset { key: payload.key.clone() })],
    })
}
//#endregion 🔖️Inverse
