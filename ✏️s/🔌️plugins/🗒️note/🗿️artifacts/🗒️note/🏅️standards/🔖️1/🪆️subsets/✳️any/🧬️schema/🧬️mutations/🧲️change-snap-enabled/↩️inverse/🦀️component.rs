//! ↩️ Inverse for `ChangeSnapEnabled`.
use super::mutation::ChangeSnapEnabled;
use crate::artifacts::note::schema::mutations::NoteMutation;
use crate::artifacts::note::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeSnapEnabled, base: &NoteSnapshot) -> Vec<NoteMutation> {
    vec![NoteMutation::ChangeSnapEnabled(ChangeSnapEnabled { new_enabled: base.snap_enabled })]
}
//#endregion 🔖️Inverse
