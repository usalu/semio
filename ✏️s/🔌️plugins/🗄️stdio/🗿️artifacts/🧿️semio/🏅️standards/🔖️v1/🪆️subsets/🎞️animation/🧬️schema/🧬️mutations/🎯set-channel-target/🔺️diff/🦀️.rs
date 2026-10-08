//! 🔺️ Diff for `SetChannelTarget`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetChannelTarget, base: &SemioAnimationSnapshot) -> protocol::MutationOutcome<SemioAnimationDiff> {
    use SemioAnimationMutation::*;
    let super::SetChannelTarget { timeline_index, index, target } = payload;
    protocol::MutationOutcome::new(diff_channel_field(*timeline_index, *index, AnimChannelDiff { target: Some(target.clone()), interpolation: None, keyframes: None }))
}
//#endregion 🔖️Diff
