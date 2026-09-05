//! ↩️ Inverse for `RemoveRun`.

use crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::{SemioTextMutation, insert_run};
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveRun, base: &SemioTextSnapshot) -> Vec<SemioTextMutation> {
    match base.runs.get(payload.index) {
        Some(run) => vec![SemioTextMutation::InsertRun(insert_run::InsertRun { index: payload.index, run: run.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
