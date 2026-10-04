//! ↩️ Inverse for `CreateAsset`.
use super::CreateAsset;
use crate::schema::mutations::DeleteAsset;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &CreateAsset, _base: &NoteSnapshot) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![NoteMutation::DeleteAsset(DeleteAsset { key: payload.key.clone() })]

    })())
}
//#endregion 🔖️Inverse
