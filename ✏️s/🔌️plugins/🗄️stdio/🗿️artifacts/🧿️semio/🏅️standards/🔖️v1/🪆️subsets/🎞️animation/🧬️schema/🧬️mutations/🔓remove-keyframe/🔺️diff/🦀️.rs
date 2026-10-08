//! 🔺️ Diff for `RemoveKeyframe`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveKeyframe, base: &SemioAnimationSnapshot) -> protocol::MutationOutcome<SemioAnimationDiff> {
    if keyframe_at(base, payload.timeline_index, payload.channel_index, payload.index).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No keyframe exists at index #{}.", payload.index), [payload.index.to_string()]);
    }
    use SemioAnimationMutation::*;
    let super::RemoveKeyframe { timeline_index, channel_index, index } = payload;
    protocol::MutationOutcome::new(diff_keyframe_collection(*timeline_index, *channel_index, IndexedTripleDiff { removed: vec![*index], ..Default::default() }))
}
//#endregion 🔖️Diff
