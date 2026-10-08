//! 🧮️ Net of one snapshot edit as animation domain leaves: the delta between the edited snapshot and the base, expressed as the
//! concrete `animation` mutations that carry the base to it. A change the vocabulary cannot express yields leaves whose fold differs from
//! the edit, which the editor's publication check refuses.

use crate::standards::v1::subsets::animation::schema::mutations::{insert_channel, insert_keyframe, insert_timeline, remove_channel, remove_keyframe, remove_timeline, set_channel_interpolation, set_channel_target, set_keyframe_time, set_keyframe_value, set_timeline_name, SemioAnimationMutation};
use crate::standards::v1::subsets::animation::schema::snapshot::{AnimChannel, AnimTimeline, SemioAnimationSnapshot};
use crate::standards::v1::subsets::base::schema::triples::{net_ordered, NetStep};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn net(base: &SemioAnimationSnapshot, next: &SemioAnimationSnapshot) -> Vec<SemioAnimationMutation> {
    let mut out = Vec::new();
    for step in net_ordered(&base.timelines, &next.timelines) {
        match step {
            NetStep::Modify { index, item } => net_timeline(index, &base.timelines[index], item, &mut out),
            NetStep::Remove { index } => out.push(SemioAnimationMutation::RemoveTimeline(remove_timeline::RemoveTimeline { index })),
            NetStep::Insert { index, item } => out.push(SemioAnimationMutation::InsertTimeline(insert_timeline::InsertTimeline { index, timeline: item.clone() })),
        }
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn net_timeline(timeline_index: usize, before: &AnimTimeline, after: &AnimTimeline, out: &mut Vec<SemioAnimationMutation>) {
    if before.name != after.name {
        out.push(SemioAnimationMutation::SetTimelineName(set_timeline_name::SetTimelineName { index: timeline_index, name: after.name.clone() }));
    }
    for step in net_ordered(&before.channels, &after.channels) {
        match step {
            NetStep::Modify { index, item } => net_channel(timeline_index, index, &before.channels[index], item, out),
            NetStep::Remove { index } => out.push(SemioAnimationMutation::RemoveChannel(remove_channel::RemoveChannel { timeline_index, index })),
            NetStep::Insert { index, item } => out.push(SemioAnimationMutation::InsertChannel(insert_channel::InsertChannel { timeline_index, index, channel: item.clone() })),
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn net_channel(timeline_index: usize, channel_index: usize, before: &AnimChannel, after: &AnimChannel, out: &mut Vec<SemioAnimationMutation>) {
    if before.target != after.target {
        out.push(SemioAnimationMutation::SetChannelTarget(set_channel_target::SetChannelTarget { timeline_index, index: channel_index, target: after.target.clone() }));
    }
    if before.interpolation != after.interpolation {
        out.push(SemioAnimationMutation::SetChannelInterpolation(set_channel_interpolation::SetChannelInterpolation { timeline_index, index: channel_index, interpolation: after.interpolation }));
    }
    for step in net_ordered(&before.keyframes, &after.keyframes) {
        match step {
            NetStep::Modify { index, item } => {
                let base_keyframe = &before.keyframes[index];
                if base_keyframe.t != item.t {
                    out.push(SemioAnimationMutation::SetKeyframeTime(set_keyframe_time::SetKeyframeTime { timeline_index, channel_index, index, t: item.t }));
                }
                if base_keyframe.value != item.value {
                    out.push(SemioAnimationMutation::SetKeyframeValue(set_keyframe_value::SetKeyframeValue { timeline_index, channel_index, index, value: item.value.clone() }));
                }
            }
            NetStep::Remove { index } => out.push(SemioAnimationMutation::RemoveKeyframe(remove_keyframe::RemoveKeyframe { timeline_index, channel_index, index })),
            NetStep::Insert { index, item } => out.push(SemioAnimationMutation::InsertKeyframe(insert_keyframe::InsertKeyframe { timeline_index, channel_index, index, keyframe: item.clone() })),
        }
    }
}
