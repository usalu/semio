//! 🔺️ Diff for `RemoveSlide`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveSlide, base: &SemioPresentationSnapshot) -> protocol::MutationOutcome<SemioPresentationDiff> {
    let super::RemoveSlide { index } = payload;
    protocol::MutationOutcome::new(diff_remove_slide(*index))
}
//#endregion 🔖️Diff
