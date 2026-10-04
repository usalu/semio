//! ↩️ Inverse for `ChangeTitle` — restores the captured BASE title (whole-document scope, always
//! present).

use crate::mutations::PlaybookMutation;
use crate::PlaybookSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ChangeTitle, base: &PlaybookSnapshot) -> Result<Vec<PlaybookMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let _ = payload;
    vec![crate::mutations::change_title::change_title_operation(base.title.clone())]

    })())
}
//#endregion 🔖️Inverse
