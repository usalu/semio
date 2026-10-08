//! 🔺️ Diff for `InsertTimeline`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::InsertTimeline, base: &SemioAnimationSnapshot) -> protocol::MutationOutcome<SemioAnimationDiff> {
    if payload.index > base.timelines.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No timeline slot exists at index #{}.", payload.index), [payload.index.to_string()]);
    }
    use SemioAnimationMutation::*;
    let super::InsertTimeline { index, timeline } = payload;
    protocol::MutationOutcome::new(SemioAnimationDiff { timelines: Some(IndexedTripleDiff { added: vec![IndexAdded { index: *index, item: timeline.clone() }], ..Default::default() }) })
}
//#endregion 🔖️Diff
