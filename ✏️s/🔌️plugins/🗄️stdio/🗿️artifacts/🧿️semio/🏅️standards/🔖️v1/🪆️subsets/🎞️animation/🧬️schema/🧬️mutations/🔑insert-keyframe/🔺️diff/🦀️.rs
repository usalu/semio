//! 🔺️ Diff for `InsertKeyframe`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::InsertKeyframe, base: &SemioAnimationSnapshot) -> protocol::MutationOutcome<SemioAnimationDiff> {
    use SemioAnimationMutation::*;
    let super::InsertKeyframe { timeline_index, channel_index, index, keyframe } = payload;
    protocol::MutationOutcome::new({
        diff_keyframe_collection(*timeline_index, *channel_index, IndexedTripleDiff { added: vec![IndexAdded { index: *index, item: keyframe.clone() }], ..Default::default() })
    })
}
//#endregion 🔖️Diff
