//! ↩️ Inverse for `ChangeGridVisible`.
use super::ChangeGridVisible;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeGridVisible, base: &NoteSnapshot) -> Result<Vec<NoteMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![NoteMutation::ChangeGridVisible(ChangeGridVisible { new_visible: base.grid_visible })]

    })())
}
//#endregion 🔖️Inverse
