//! 🔺️ Diff for `RemoveTimeline`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveTimeline, base: &SemioAnimationSnapshot) -> protocol::MutationOutcome<SemioAnimationDiff> {
    use SemioAnimationMutation::*;
    let super::RemoveTimeline { index } = payload;
    protocol::MutationOutcome::new(SemioAnimationDiff { timelines: Some(IndexedTripleDiff { removed: vec![*index], ..Default::default() }) })
}
//#endregion 🔖️Diff
