//! ↩️ Inverse for `ChangePencilWidth`.
use super::ChangePencilWidth;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangePencilWidth, base: &NoteSnapshot) -> Vec<NoteMutation> {
    vec![NoteMutation::ChangePencilWidth(ChangePencilWidth { new_width: base.pencil_width })]
}
//#endregion 🔖️Inverse
