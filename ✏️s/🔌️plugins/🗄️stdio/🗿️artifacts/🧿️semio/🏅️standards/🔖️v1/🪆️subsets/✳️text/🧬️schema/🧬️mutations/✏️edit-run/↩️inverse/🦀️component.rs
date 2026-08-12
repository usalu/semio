//! ↩️ `edit-run` — undo restores BASE's content at that index; out-of-range BASE index ⇒
//! `Vec::new()`.

use super::mutation::EditRun;
use crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::SemioTextMutation;
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &EditRun, base: &SemioTextSnapshot) -> Vec<SemioTextMutation> {
    match base.runs.get(payload.index) {
        Some(run) => vec![SemioTextMutation::EditRun(EditRun { index: payload.index, new_content: run.content.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
