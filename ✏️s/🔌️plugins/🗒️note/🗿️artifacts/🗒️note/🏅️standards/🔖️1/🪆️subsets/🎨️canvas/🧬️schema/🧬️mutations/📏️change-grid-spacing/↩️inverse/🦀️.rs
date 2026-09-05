//! ↩️ Inverse for `ChangeGridSpacing`.
use super::ChangeGridSpacing;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeGridSpacing, base: &NoteSnapshot) -> Vec<NoteMutation> {
    vec![NoteMutation::ChangeGridSpacing(ChangeGridSpacing { new_spacing: base.grid_spacing })]
}
//#endregion 🔖️Inverse
