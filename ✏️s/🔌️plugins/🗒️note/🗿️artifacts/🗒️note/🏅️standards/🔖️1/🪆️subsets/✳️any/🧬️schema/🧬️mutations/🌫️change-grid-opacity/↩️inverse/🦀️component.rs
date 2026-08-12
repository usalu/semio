//! ↩️ Inverse for `ChangeGridOpacity`.
use super::mutation::ChangeGridOpacity;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeGridOpacity, base: &NoteSnapshot) -> Vec<NoteMutation> {
    vec![NoteMutation::ChangeGridOpacity(ChangeGridOpacity { new_opacity: base.grid_opacity })]
}
//#endregion 🔖️Inverse
