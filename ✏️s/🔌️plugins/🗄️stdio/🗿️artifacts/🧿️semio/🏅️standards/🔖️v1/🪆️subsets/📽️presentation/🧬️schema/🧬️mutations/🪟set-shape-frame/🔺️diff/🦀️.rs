//! 🔺️ Diff for `SetShapeFrame`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetShapeFrame, base: &SemioPresentationSnapshot) -> protocol::MutationOutcome<SemioPresentationDiff> {
    if shape_at(base, payload.slide_index, payload.shape_index).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No shape exists at index #{}.", payload.shape_index), [payload.shape_index.to_string()]);
    }
    let super::SetShapeFrame { slide_index, shape_index, frame } = payload;
    protocol::MutationOutcome::new(diff_set_shape_frame(base, *slide_index, *shape_index, *frame))
}
//#endregion 🔖️Diff
