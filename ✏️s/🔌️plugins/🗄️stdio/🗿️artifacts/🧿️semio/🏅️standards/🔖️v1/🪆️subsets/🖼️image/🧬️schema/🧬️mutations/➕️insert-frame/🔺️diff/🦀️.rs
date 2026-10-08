//! 🔺️ Diff for `InsertFrame`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::InsertFrame, base: &SemioImageSnapshot) -> protocol::MutationOutcome<SemioImageDiff> {
    let super::InsertFrame { index, frame } = payload;
    let clamped = (*index).min(base.frames.len());
    let outcome = protocol::MutationOutcome::new(SemioImageDiff { frames: Some(SemioImageFramesDiff { added: vec![IndexAdded { index: clamped, item: frame.clone() }], ..Default::default() }), ..Default::default() });
    if clamped == *index {
        outcome
    } else {
        outcome.warning("mutation.clamped", format!("Insert index {index} was past the end; clamped to {clamped}."))
    }
}
//#endregion 🔖️Diff
