//! 🔺️ Diff for `RemoveShape`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveShape, base: &SemioPresentationSnapshot) -> protocol::MutationOutcome<SemioPresentationDiff> {
    let super::RemoveShape { slide_index, shape_index } = payload;
    protocol::MutationOutcome::new(diff_remove_shape(*slide_index, *shape_index))
}
//#endregion 🔖️Diff
