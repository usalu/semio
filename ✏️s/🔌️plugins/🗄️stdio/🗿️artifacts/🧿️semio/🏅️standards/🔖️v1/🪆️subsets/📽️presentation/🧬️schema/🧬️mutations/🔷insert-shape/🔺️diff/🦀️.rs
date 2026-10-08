//! 🔺️ Diff for `InsertShape`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::InsertShape, base: &SemioPresentationSnapshot) -> protocol::MutationOutcome<SemioPresentationDiff> {
    let super::InsertShape { slide_index, shape_index, shape } = payload;
    protocol::MutationOutcome::new(diff_insert_shape(*slide_index, *shape_index, shape.clone()))
}
//#endregion 🔖️Diff
