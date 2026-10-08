//! 🔺️ Diff for `SetChannelInterpolation`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetChannelInterpolation, base: &SemioAnimationSnapshot) -> protocol::MutationOutcome<SemioAnimationDiff> {
    use SemioAnimationMutation::*;
    let super::SetChannelInterpolation { timeline_index, index, interpolation } = payload;
    protocol::MutationOutcome::new({
        diff_channel_field(*timeline_index, *index, AnimChannelDiff { target: None, interpolation: Some(*interpolation), keyframes: None })
    })
}
//#endregion 🔖️Diff
