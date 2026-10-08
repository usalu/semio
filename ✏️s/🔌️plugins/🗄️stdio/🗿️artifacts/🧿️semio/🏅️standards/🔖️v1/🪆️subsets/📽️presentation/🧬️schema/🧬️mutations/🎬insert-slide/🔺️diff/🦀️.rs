//! 🔺️ Diff for `InsertSlide`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::InsertSlide, base: &SemioPresentationSnapshot) -> protocol::MutationOutcome<SemioPresentationDiff> {
    if payload.index > base.slides.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No slide slot exists at index #{}.", payload.index), [payload.index.to_string()]);
    }
    let super::InsertSlide { index, slide } = payload;
    protocol::MutationOutcome::new(diff_insert_slide(*index, slide.clone()))
}
//#endregion 🔖️Diff
