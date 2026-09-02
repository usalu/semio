//! ↩️ Inverse for `ChangePencilWidth`.
use super::ChangePencilWidth;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangePencilWidth, base: &NoteSnapshot) -> Vec<NoteMutation> {
    vec![NoteMutation::ChangePencilWidth(ChangePencilWidth { new_width: base.pencil_width })]
}
//#endregion 🔖️Inverse
