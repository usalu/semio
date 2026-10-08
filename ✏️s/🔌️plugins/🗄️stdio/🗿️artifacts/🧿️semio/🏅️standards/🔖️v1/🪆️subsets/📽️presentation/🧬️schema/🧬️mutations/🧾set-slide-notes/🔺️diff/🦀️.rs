//! 🔺️ Diff for `SetSlideNotes`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetSlideNotes, base: &SemioPresentationSnapshot) -> protocol::MutationOutcome<SemioPresentationDiff> {
    if payload.index >= base.slides.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No slide exists at index #{}.", payload.index), [payload.index.to_string()]);
    }
    let super::SetSlideNotes { index, notes } = payload;
    protocol::MutationOutcome::new(diff_set_slide_notes(base, *index, notes))
}
//#endregion 🔖️Diff
