//! ↩️ Inverse for `ChangePencilWidth`.
use super::mutation::ChangePencilWidth;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangePencilWidth, base: &NoteSnapshot) -> Vec<NoteMutation> {
    vec![NoteMutation::ChangePencilWidth(ChangePencilWidth { new_width: base.pencil_width })]
}
//#endregion 🔖️Inverse
