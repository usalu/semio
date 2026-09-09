//! ↩️ Inverse for `AddMark`.

use crate::standards::v1::subsets::text::schema::mutations::{remove_mark, SemioTextMutation};
use crate::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::AddMark, base: &SemioTextSnapshot) -> Vec<SemioTextMutation> {
    match base.runs.get(payload.run_index) {
        Some(run) => {
            let at = payload.index.min(run.marks.len());
            vec![SemioTextMutation::RemoveMark(remove_mark::RemoveMark { run_index: payload.run_index, index: at })]
        }
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
