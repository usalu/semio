//! ↩️ Inverse for `RemoveMark`.

use crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::{SemioTextMutation, add_mark};
use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveMark, base: &SemioTextSnapshot) -> Vec<SemioTextMutation> {
    match base.runs.get(payload.run_index).and_then(|run| run.marks.get(payload.index)) {
        Some(mark) => vec![SemioTextMutation::AddMark(add_mark::AddMark { run_index: payload.run_index, index: payload.index, mark: mark.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
