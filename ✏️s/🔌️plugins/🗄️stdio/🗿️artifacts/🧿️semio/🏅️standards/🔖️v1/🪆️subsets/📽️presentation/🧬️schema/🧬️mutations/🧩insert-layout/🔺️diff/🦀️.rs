//! 🔺️ Diff for `InsertLayout`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::InsertLayout, base: &SemioPresentationSnapshot) -> protocol::MutationOutcome<SemioPresentationDiff> {
    let super::InsertLayout { layout } = payload;
    protocol::MutationOutcome::new(diff_insert_layout(layout.clone()))
}
//#endregion 🔖️Diff
