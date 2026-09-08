//! ↩️ Inverse for `RenameNote`.
use super::RenameNote;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &RenameNote, base: &NoteSnapshot) -> Vec<NoteMutation> {
    vec![NoteMutation::RenameNote(RenameNote { new_title: base.title.clone() })]
}
//#endregion 🔖️Inverse
