//! 🔺️ Diff for `InsertShape`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::InsertShape, base: &SemioPresentationSnapshot) -> protocol::MutationOutcome<SemioPresentationDiff> {
    if base.slides.get(payload.slide_index).is_none_or(|s| payload.shape_index > s.shapes.len()) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No shape slot exists at index #{}.", payload.shape_index), [payload.shape_index.to_string()]);
    }
    let super::InsertShape { slide_index, shape_index, shape } = payload;
    protocol::MutationOutcome::new(diff_insert_shape(*slide_index, *shape_index, shape.clone()))
}
//#endregion 🔖️Diff
