//! ↩️ Inverse for `ChangeSnapGridSpacing`.
use super::ChangeSnapGridSpacing;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeSnapGridSpacing, base: &NoteSnapshot) -> Vec<NoteMutation> {
    vec![NoteMutation::ChangeSnapGridSpacing(ChangeSnapGridSpacing { new_spacing: base.snap_grid_spacing })]
}
//#endregion 🔖️Inverse
