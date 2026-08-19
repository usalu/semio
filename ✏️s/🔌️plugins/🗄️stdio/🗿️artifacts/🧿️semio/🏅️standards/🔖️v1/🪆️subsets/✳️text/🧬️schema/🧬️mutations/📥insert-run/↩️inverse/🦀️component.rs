//! ↩️ `insert-run` — undo is `remove-run` at the (clamped) FINAL-state index the run landed at,
//! which is also a valid BASE-state index for the follow-up removal.

use super::mutation::InsertRun;
use crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::remove_run;
use crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::SemioTextMutation;
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &InsertRun, base: &SemioTextSnapshot) -> Vec<SemioTextMutation> {
    let at = payload.index.min(base.runs.len());
    vec![SemioTextMutation::RemoveRun(remove_run::mutation::RemoveRun { index: at })]
}
//#endregion 🔖️Inverse
