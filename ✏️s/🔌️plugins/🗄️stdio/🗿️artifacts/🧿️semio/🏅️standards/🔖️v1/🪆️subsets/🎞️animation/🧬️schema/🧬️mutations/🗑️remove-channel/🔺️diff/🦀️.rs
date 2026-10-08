//! 🔺️ Diff for `RemoveChannel`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveChannel, base: &SemioAnimationSnapshot) -> protocol::MutationOutcome<SemioAnimationDiff> {
    use SemioAnimationMutation::*;
    let super::RemoveChannel { timeline_index, index } = payload;
    protocol::MutationOutcome::new(diff_channel_collection(*timeline_index, IndexedTripleDiff { removed: vec![*index], ..Default::default() }))
}
//#endregion 🔖️Diff
