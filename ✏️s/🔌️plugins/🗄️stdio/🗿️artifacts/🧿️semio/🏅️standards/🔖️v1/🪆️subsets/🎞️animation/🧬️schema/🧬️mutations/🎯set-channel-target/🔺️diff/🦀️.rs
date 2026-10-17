//! 🔺️ Diff for `SetChannelTarget`.

#[allow(unused_imports)]
use crate::schema::diff::AnimChannelDiff;
#[allow(unused_imports)]
use crate::schema::diff::SemioAnimationDiff;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioAnimationSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::diff_channel_field;
#[allow(unused_imports)]
use crate::schema::mutations::support::timeline_at;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetChannelTarget, base: &SemioAnimationSnapshot) -> protocol::MutationOutcome<SemioAnimationDiff> {
    if timeline_at(base, payload.timeline_index).is_none_or(|t| payload.index >= t.channels.len()) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No channel exists at index #{}.", payload.index), [payload.index.to_string()]);
    }
    
    let super::SetChannelTarget { timeline_index, index, target } = payload;
    protocol::MutationOutcome::new(diff_channel_field(*timeline_index, *index, AnimChannelDiff { target: Some(target.clone()), interpolation: None, keyframes: None }))
}
//#endregion 🔖️Diff
