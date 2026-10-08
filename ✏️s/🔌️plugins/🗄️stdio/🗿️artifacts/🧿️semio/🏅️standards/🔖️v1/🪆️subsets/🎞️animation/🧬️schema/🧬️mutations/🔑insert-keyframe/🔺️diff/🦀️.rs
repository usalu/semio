//! 🔺️ Diff for `InsertKeyframe`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::InsertKeyframe, base: &SemioAnimationSnapshot) -> protocol::MutationOutcome<SemioAnimationDiff> {
    if channel_at(base, payload.timeline_index, payload.channel_index).is_none_or(|c| payload.index > c.keyframes.len()) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No keyframe slot exists at index #{}.", payload.index), [payload.index.to_string()]);
    }
    use SemioAnimationMutation::*;
    let super::InsertKeyframe { timeline_index, channel_index, index, keyframe } = payload;
    protocol::MutationOutcome::new({
        diff_keyframe_collection(*timeline_index, *channel_index, IndexedTripleDiff { added: vec![IndexAdded { index: *index, item: keyframe.clone() }], ..Default::default() })
    })
}
//#endregion 🔖️Diff
