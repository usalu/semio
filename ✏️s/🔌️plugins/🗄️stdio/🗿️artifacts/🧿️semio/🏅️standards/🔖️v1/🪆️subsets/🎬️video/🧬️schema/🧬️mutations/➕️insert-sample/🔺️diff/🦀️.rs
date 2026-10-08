//! 🔺️ Diff for `InsertSample`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::InsertSample, base: &SemioVideoSnapshot) -> protocol::MutationOutcome<SemioVideoDiff> {
    let super::InsertSample { stream_index, index, sample } = payload;
    protocol::MutationOutcome::new(diff_insert_sample(*stream_index, *index, sample.clone()))
}
//#endregion 🔖️Diff
