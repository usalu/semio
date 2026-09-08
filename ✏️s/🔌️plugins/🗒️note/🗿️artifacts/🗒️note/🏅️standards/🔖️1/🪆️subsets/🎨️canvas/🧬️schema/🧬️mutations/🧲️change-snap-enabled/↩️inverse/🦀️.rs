//! ↩️ Inverse for `ChangeSnapEnabled`.
use super::ChangeSnapEnabled;
use crate::schema::mutations::NoteMutation;
use crate::NoteSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeSnapEnabled, base: &NoteSnapshot) -> Vec<NoteMutation> {
    vec![NoteMutation::ChangeSnapEnabled(ChangeSnapEnabled { new_enabled: base.snap_enabled })]
}
//#endregion 🔖️Inverse
