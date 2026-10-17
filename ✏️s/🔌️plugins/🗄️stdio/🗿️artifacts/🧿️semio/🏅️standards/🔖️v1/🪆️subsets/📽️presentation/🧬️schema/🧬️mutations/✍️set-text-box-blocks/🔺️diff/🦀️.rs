//! 🔺️ Diff for `SetTextBoxBlocks`.

#[allow(unused_imports)]
use crate::schema::diff::SemioPresentationDiff;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioPresentationSnapshot;
#[allow(unused_imports)]
use crate::schema::diff::diff_set_textbox_blocks;
#[allow(unused_imports)]
use crate::schema::mutations::support::shape_at;

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
