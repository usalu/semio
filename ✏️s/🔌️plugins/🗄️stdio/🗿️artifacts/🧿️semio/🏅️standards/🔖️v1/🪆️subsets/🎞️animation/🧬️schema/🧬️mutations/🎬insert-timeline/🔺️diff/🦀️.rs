//! 🔺️ Diff for `InsertTimeline`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::InsertTimeline, base: &SemioAnimationSnapshot) -> protocol::MutationOutcome<SemioAnimationDiff> {
    use SemioAnimationMutation::*;
    let super::InsertTimeline { index, timeline } = payload;
    protocol::MutationOutcome::new(SemioAnimationDiff { timelines: Some(IndexedTripleDiff { added: vec![IndexAdded { index: *index, item: timeline.clone() }], ..Default::default() }) })
}
//#endregion 🔖️Diff
