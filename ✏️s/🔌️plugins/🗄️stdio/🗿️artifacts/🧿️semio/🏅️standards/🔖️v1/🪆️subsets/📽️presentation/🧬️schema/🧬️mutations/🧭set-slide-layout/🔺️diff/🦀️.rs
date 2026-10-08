//! 🔺️ Diff for `SetSlideLayout`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetSlideLayout, base: &SemioPresentationSnapshot) -> protocol::MutationOutcome<SemioPresentationDiff> {
    if payload.index >= base.slides.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No slide exists at index #{}.", payload.index), [payload.index.to_string()]);
    }
    let super::SetSlideLayout { index, layout_id } = payload;
    protocol::MutationOutcome::new(diff_set_slide_layout(base, *index, layout_id.clone()))
}
//#endregion 🔖️Diff
