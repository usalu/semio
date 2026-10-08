//! 🔺️ Diff for `RemoveTimeline`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveTimeline, base: &SemioAnimationSnapshot) -> protocol::MutationOutcome<SemioAnimationDiff> {
    if timeline_at(base, payload.index).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No timeline exists at index #{}.", payload.index), [payload.index.to_string()]);
    }
    use SemioAnimationMutation::*;
    let super::RemoveTimeline { index } = payload;
    protocol::MutationOutcome::new(SemioAnimationDiff { timelines: Some(IndexedTripleDiff { removed: vec![*index], ..Default::default() }) })
}
//#endregion 🔖️Diff
