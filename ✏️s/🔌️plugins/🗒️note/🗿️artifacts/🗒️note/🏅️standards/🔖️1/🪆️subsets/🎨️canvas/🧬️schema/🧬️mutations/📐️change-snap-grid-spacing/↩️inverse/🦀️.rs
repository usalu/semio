//! ↩️ Inverse for `ChangeSnapGridSpacing`.
use super::ChangeSnapGridSpacing;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeSnapGridSpacing, base: &NoteSnapshot) -> Vec<NoteMutation> {
    vec![NoteMutation::ChangeSnapGridSpacing(ChangeSnapGridSpacing { new_spacing: base.snap_grid_spacing })]
}
//#endregion 🔖️Inverse
