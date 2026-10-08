//! 🔺️ Diff for `SetShapeFrame`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetShapeFrame, base: &SemioPresentationSnapshot) -> protocol::MutationOutcome<SemioPresentationDiff> {
    let super::SetShapeFrame { slide_index, shape_index, frame } = payload;
    protocol::MutationOutcome::new(diff_set_shape_frame(base, *slide_index, *shape_index, *frame))
}
//#endregion 🔖️Diff
