//! 🔺️ Diff for `SetTextBoxBlocks`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetTextBoxBlocks, base: &SemioPresentationSnapshot) -> protocol::MutationOutcome<SemioPresentationDiff> {
    let super::SetTextBoxBlocks { slide_index, shape_index, blocks } = payload;
    protocol::MutationOutcome::new(diff_set_textbox_blocks(base, *slide_index, *shape_index, blocks))
}
//#endregion 🔖️Diff
