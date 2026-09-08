//! ↩️ Inverse for `ChangeGridOpacity`.
use super::ChangeGridOpacity;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeGridOpacity, base: &NoteSnapshot) -> Vec<NoteMutation> {
    vec![NoteMutation::ChangeGridOpacity(ChangeGridOpacity { new_opacity: base.grid_opacity })]
}
//#endregion 🔖️Inverse
