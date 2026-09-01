//! ↩️ Inverse for `RenameNote`.
use super::RenameNote;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &RenameNote, base: &NoteSnapshot) -> Vec<NoteMutation> {
    vec![NoteMutation::RenameNote(RenameNote { new_title: base.title.clone() })]
}
//#endregion 🔖️Inverse
