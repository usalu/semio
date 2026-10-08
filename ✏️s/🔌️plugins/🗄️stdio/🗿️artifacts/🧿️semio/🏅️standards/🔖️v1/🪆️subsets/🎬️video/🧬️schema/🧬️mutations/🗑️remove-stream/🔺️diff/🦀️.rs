//! 🔺️ Diff for `RemoveStream`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveStream, base: &SemioVideoSnapshot) -> protocol::MutationOutcome<SemioVideoDiff> {
    let super::RemoveStream { index } = payload;
    protocol::MutationOutcome::new(diff_remove_stream(*index))
}
//#endregion 🔖️Diff
