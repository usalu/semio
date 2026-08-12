//! ↩️ Inverse for `RenameNote`.
use super::mutation::RenameNote;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &RenameNote, base: &NoteSnapshot) -> Vec<NoteMutation> {
    vec![NoteMutation::RenameNote(RenameNote { new_title: base.title.clone() })]
}
//#endregion 🔖️Inverse
