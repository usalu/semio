//! 🔺️ Diff for `SetTextBoxBlocks`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetTextBoxBlocks, base: &SemioPresentationSnapshot) -> protocol::MutationOutcome<SemioPresentationDiff> {
    if shape_at(base, payload.slide_index, payload.shape_index).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No shape exists at index #{}.", payload.shape_index), [payload.shape_index.to_string()]);
    }
    let super::SetTextBoxBlocks { slide_index, shape_index, blocks } = payload;
    protocol::MutationOutcome::new(diff_set_textbox_blocks(base, *slide_index, *shape_index, blocks))
}
//#endregion 🔖️Diff
