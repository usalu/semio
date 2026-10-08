//! 🔺️ Diff for `MoveFrame`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::MoveFrame, base: &SemioImageSnapshot) -> protocol::MutationOutcome<SemioImageDiff> {
    let super::MoveFrame { from, to } = payload;
    if *from >= base.frames.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Frame index {from} does not exist."), [from.to_string()]);
    }
    if *to >= base.frames.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Frame index {to} does not exist."), [to.to_string()]);
    }
    if from == to {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Frame {from} is already at this position."));
    }
    protocol::MutationOutcome::new({
        let frames = base.frames.get(*from).map(|item| SemioImageFramesDiff { removed: vec![*from], added: vec![IndexAdded { index: *to, item: item.clone() }], ..Default::default() });
        SemioImageDiff { frames, ..Default::default() }
    })
}
//#endregion 🔖️Diff
