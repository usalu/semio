//! ↩️ `remove-mark` — undo re-attaches the captured mark at the same BASE-state index; an absent
//! `run_index`/`index` ⇒ `Vec::new()`.

use super::mutation::RemoveMark;
use crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::add_mark;
use crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::SemioTextMutation;
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &RemoveMark, base: &SemioTextSnapshot) -> Vec<SemioTextMutation> {
    match base.runs.get(payload.run_index).and_then(|run| run.marks.get(payload.index)) {
        Some(mark) => vec![SemioTextMutation::AddMark(add_mark::mutation::AddMark { run_index: payload.run_index, index: payload.index, mark: mark.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
