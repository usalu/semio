//! ↩️ Inverse for `ChangeGridOpacity`.
use super::ChangeGridOpacity;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeGridOpacity, base: &NoteSnapshot) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![NoteMutation::ChangeGridOpacity(ChangeGridOpacity { new_opacity: base.grid_opacity })]

    })())
}
//#endregion 🔖️Inverse
