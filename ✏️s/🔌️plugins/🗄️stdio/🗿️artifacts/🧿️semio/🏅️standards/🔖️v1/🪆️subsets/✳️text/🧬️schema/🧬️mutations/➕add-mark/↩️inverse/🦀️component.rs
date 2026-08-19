//! ↩️ `add-mark` — undo is `remove-mark` at the (clamped) FINAL-state index the mark landed at,
//! which is also a valid BASE-state index for the follow-up removal; an absent `run_index` ⇒
//! `Vec::new()`.

use super::mutation::AddMark;
use crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::remove_mark;
use crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::SemioTextMutation;
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &AddMark, base: &SemioTextSnapshot) -> Vec<SemioTextMutation> {
    match base.runs.get(payload.run_index) {
        Some(run) => {
            let at = payload.index.min(run.marks.len());
            vec![SemioTextMutation::RemoveMark(remove_mark::mutation::RemoveMark { run_index: payload.run_index, index: at })]
        }
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
