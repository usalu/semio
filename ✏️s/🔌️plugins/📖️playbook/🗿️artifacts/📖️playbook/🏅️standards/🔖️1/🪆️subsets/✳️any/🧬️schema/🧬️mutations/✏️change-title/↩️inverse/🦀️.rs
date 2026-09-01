//! ↩️ Inverse for `ChangeTitle` — restores the captured BASE title (whole-document scope, always
//! present).

use crate::artifacts::playbook::mutations::PlaybookMutation;
use crate::artifacts::playbook::PlaybookSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ChangeTitle, base: &PlaybookSnapshot) -> Vec<PlaybookMutation> {
    let _ = payload;
    vec![crate::artifacts::playbook::mutations::change_title::change_title_operation(base.title.clone())]
}
//#endregion 🔖️Inverse
