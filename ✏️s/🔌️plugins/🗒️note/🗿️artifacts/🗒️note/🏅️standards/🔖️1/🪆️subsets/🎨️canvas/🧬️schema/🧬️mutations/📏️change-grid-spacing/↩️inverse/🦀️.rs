//! ↩️ Inverse for `ChangeGridSpacing`.
use super::ChangeGridSpacing;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeGridSpacing, base: &NoteSnapshot) -> Vec<NoteMutation> {
    vec![NoteMutation::ChangeGridSpacing(ChangeGridSpacing { new_spacing: base.grid_spacing })]
}
//#endregion 🔖️Inverse
