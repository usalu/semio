//! ↩️ Inverse for `ChangeGridSpacing`.
use super::mutation::ChangeGridSpacing;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ChangeGridSpacing, base: &NoteSnapshot) -> Vec<NoteMutation> {
    vec![NoteMutation::ChangeGridSpacing(ChangeGridSpacing { new_spacing: base.grid_spacing })]
}
//#endregion 🔖️Inverse
