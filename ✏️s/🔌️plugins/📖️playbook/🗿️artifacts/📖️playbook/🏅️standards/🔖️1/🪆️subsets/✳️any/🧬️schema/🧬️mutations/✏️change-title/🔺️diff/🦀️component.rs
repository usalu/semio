//! 🔺️ Sparse diff builder for `ChangeTitle` — a real single-field patch (never a whole-snapshot
//! capture).
use crate::artifacts::playbook::{PlaybookDiff, PlaybookSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeTitle, _base: &PlaybookSnapshot) -> PlaybookDiff {
    PlaybookDiff { title: Some(payload.new_title.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
