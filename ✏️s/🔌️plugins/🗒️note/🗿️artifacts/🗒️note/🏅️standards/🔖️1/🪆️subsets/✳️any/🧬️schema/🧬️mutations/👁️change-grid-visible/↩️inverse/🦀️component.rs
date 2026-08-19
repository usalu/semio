//! ↩️ Inverse for `ChangeGridVisible`.
use super::mutation::ChangeGridVisible;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeGridVisible, base: &NoteSnapshot) -> Vec<NoteMutation> {
    vec![NoteMutation::ChangeGridVisible(ChangeGridVisible { new_visible: base.grid_visible })]
}
//#endregion 🔖️Inverse
