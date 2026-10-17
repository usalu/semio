//! 🧰️ `SemioAnimationMutation` support: the diff builders and base accessors its mutation leaves call, hosted below the leaves so that no leaf has to import the aggregate module.

use crate::schema::snapshot::AnimChannel;
use crate::schema::diff::AnimChannelDiff;
use crate::schema::snapshot::AnimKeyframe;
use crate::schema::diff::AnimKeyframeDiff;
use crate::schema::snapshot::AnimTimeline;
use crate::schema::diff::AnimTimelineDiff;
use semio_s_artifact_stdio_semio_base::schema::triples::IndexModified;
use semio_s_artifact_stdio_semio_base::schema::triples::IndexedTripleDiff;
use crate::schema::diff::SemioAnimationDiff;
use crate::schema::snapshot::SemioAnimationSnapshot;

/// 🧱️ Wraps a per-timeline `AnimTimelineDiff` into a full `SemioAnimationDiff` — the innermost
/// layer of the nested-diff tree every non-collection-root mutation ultimately builds on.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_timeline_field(index: usize, diff: AnimTimelineDiff) -> SemioAnimationDiff {
    SemioAnimationDiff { timelines: Some(IndexedTripleDiff { modified: vec![IndexModified { index, diff }], ..Default::default() }) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_channel_collection(timeline_index: usize, channels: IndexedTripleDiff<AnimChannelDiff, AnimChannel>) -> SemioAnimationDiff {
    diff_timeline_field(timeline_index, AnimTimelineDiff { name: None, channels: Some(channels) })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_channel_field(timeline_index: usize, index: usize, diff: AnimChannelDiff) -> SemioAnimationDiff {
    diff_channel_collection(timeline_index, IndexedTripleDiff { modified: vec![IndexModified { index, diff }], ..Default::default() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_keyframe_collection(timeline_index: usize, channel_index: usize, keyframes: IndexedTripleDiff<AnimKeyframeDiff, AnimKeyframe>) -> SemioAnimationDiff {
    diff_channel_field(timeline_index, channel_index, AnimChannelDiff { target: None, interpolation: None, keyframes: Some(keyframes) })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_keyframe_field(timeline_index: usize, channel_index: usize, index: usize, diff: AnimKeyframeDiff) -> SemioAnimationDiff {
    diff_keyframe_collection(timeline_index, channel_index, IndexedTripleDiff { modified: vec![IndexModified { index, diff }], ..Default::default() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn timeline_at(base: &SemioAnimationSnapshot, i: usize) -> Option<&AnimTimeline> {
    base.timelines.get(i)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn channel_at(base: &SemioAnimationSnapshot, ti: usize, ci: usize) -> Option<&AnimChannel> {
    base.timelines.get(ti).and_then(|t| t.channels.get(ci))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn keyframe_at(base: &SemioAnimationSnapshot, ti: usize, ci: usize, ki: usize) -> Option<&AnimKeyframe> {
    base.timelines.get(ti).and_then(|t| t.channels.get(ci)).and_then(|c| c.keyframes.get(ki))
}
