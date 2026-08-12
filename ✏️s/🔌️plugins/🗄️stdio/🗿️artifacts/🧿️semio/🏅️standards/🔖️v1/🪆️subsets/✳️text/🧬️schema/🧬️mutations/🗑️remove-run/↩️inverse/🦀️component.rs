//! ↩️ `remove-run` — undo re-inserts the captured run at the same BASE-state index; out-of-range
//! BASE index ⇒ `Vec::new()` (nothing was removed, nothing to undo).

use super::mutation::RemoveRun;
use crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::insert_run;
use crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::SemioTextMutation;
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &RemoveRun, base: &SemioTextSnapshot) -> Vec<SemioTextMutation> {
    match base.runs.get(payload.index) {
        Some(run) => vec![SemioTextMutation::InsertRun(insert_run::mutation::InsertRun { index: payload.index, run: run.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
