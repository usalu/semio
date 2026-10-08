//! 🔺️ Diff for `SetTimelineName`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetTimelineName, base: &SemioAnimationSnapshot) -> protocol::MutationOutcome<SemioAnimationDiff> {
    if timeline_at(base, payload.index).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No timeline exists at index #{}.", payload.index), [payload.index.to_string()]);
    }
    use SemioAnimationMutation::*;
    let super::SetTimelineName { index, name } = payload;
    protocol::MutationOutcome::new(diff_timeline_field(*index, AnimTimelineDiff { name: Some(name.clone()), channels: None }))
}
//#endregion 🔖️Diff
