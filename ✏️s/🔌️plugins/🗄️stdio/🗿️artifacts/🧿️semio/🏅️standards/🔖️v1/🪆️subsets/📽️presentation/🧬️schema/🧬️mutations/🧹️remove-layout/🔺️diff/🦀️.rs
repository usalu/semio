//! 🔺️ Diff for `RemoveLayout`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveLayout, base: &SemioPresentationSnapshot) -> protocol::MutationOutcome<SemioPresentationDiff> {
    let super::RemoveLayout { id } = payload;
    protocol::MutationOutcome::new(diff_remove_layout(id))
}
//#endregion 🔖️Diff
