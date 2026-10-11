//! 🧬️ SemioAnimationMutation — full named-variant vocabulary (gif 89a / docx precedent), replacing
//! the W1b `SetSnapshot`-only scaffold. Every variant's `diff()`/`inverse()` is HAND-WRITTEN
//! (apply-and-capture is banned per `🧬️schema-design.md`'s svg infinite-recursion warning) —
//! `diff()` builds the exact sparse `SemioAnimationDiff` directly via the `diff_*` helpers below,
//! never by diffing a mutated clone against `base`.
//!
//! 🪆️ Mutation-leaf migration (ticket 26/08/12/SEMANTIC-MUTATIONS-OVERHAUL): each variant now wraps
//! its own `dsl::MutationLeaf` payload type (`🧬️mutations/<emoji><kind>/🦀️.rs`), and
//! `#[derive(dsl::Mutations)]` synthesizes `DESCRIPTORS`/`descriptor()` from that leaf roster —
//! required by `protocol::Mutation<P>` (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:105`).
//! `NoMutation` is dropped: the derive requires every variant to wrap exactly one leaf payload, and
//! `no` is not an approved semantic verb.

use crate::standards::v1::subsets::animation::schema::diff::{AnimChannelDiff, AnimKeyframeDiff, AnimTimelineDiff, SemioAnimationDiff};
use crate::standards::v1::subsets::animation::schema::snapshot::{AnimChannel, AnimInterpolation, AnimKeyframe, AnimTarget, AnimTimeline, AnimValue, SemioAnimationSnapshot};
use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff};
use protocol::Mutation;
/// 🔧️ `MutationDiff` in scope for the `#[cfg(test)] mod tests` block below.
#[cfg(test)]
use protocol::MutationDiff;


//#region 🔖️Mutation
#[path = "📻insert-channel/🦀️.rs"]
pub mod insert_channel;
#[path = "🔑insert-keyframe/🦀️.rs"]
pub mod insert_keyframe;
#[path = "🎬insert-timeline/🦀️.rs"]
pub mod insert_timeline;
#[path = "🗑️remove-channel/🦀️.rs"]
pub mod remove_channel;
#[path = "🔓remove-keyframe/🦀️.rs"]
pub mod remove_keyframe;
#[path = "🧹remove-timeline/🦀️.rs"]
pub mod remove_timeline;
#[path = "📈set-channel-interpolation/🦀️.rs"]
pub mod set_channel_interpolation;
#[path = "🎯set-channel-target/🦀️.rs"]
pub mod set_channel_target;
#[path = "🕐set-keyframe-time/🦀️.rs"]
pub mod set_keyframe_time;
#[path = "🔢set-keyframe-value/🦀️.rs"]
pub mod set_keyframe_value;
//#region 🔖️Leaves
#[path = "🏷️set-timeline-name/🦀️.rs"]
pub mod set_timeline_name;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = SemioAnimationSnapshot, diff = SemioAnimationDiff, schema = "SemioAnimationMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum SemioAnimationMutation {
    InsertTimeline(insert_timeline::InsertTimeline),
    RemoveTimeline(remove_timeline::RemoveTimeline),
    /// 🏷️ `name: None` clears the timeline's display name.
    SetTimelineName(set_timeline_name::SetTimelineName),
    InsertChannel(insert_channel::InsertChannel),
    RemoveChannel(remove_channel::RemoveChannel),
    SetChannelTarget(set_channel_target::SetChannelTarget),
    SetChannelInterpolation(set_channel_interpolation::SetChannelInterpolation),
    InsertKeyframe(insert_keyframe::InsertKeyframe),
    RemoveKeyframe(remove_keyframe::RemoveKeyframe),
    SetKeyframeTime(set_keyframe_time::SetKeyframeTime),
    SetKeyframeValue(set_keyframe_value::SetKeyframeValue),
}

/// 🏷️ The declared kebab-case mutation vocabulary of `s.stdio.semio.animation`, in enum
/// declaration order — what the `🎞️mutate-semio-animation` case's completeness gate counts against
/// and what `../../🔮️oracles/🔣️.json`'s catalog repeats. Unlike its audio/video siblings
/// this subset's wire keywords are the two-letter `TEXT_KEYWORDS` heads (`IT`, `KV`, …), so the two
/// tables are related only by position; `kinds_match_the_enum_and_the_catalog` below asserts that
/// positional agreement rather than string equality.
pub const KINDS: &[&str] =
    &["insert-timeline", "remove-timeline", "set-timeline-name", "insert-channel", "remove-channel", "set-channel-target", "set-channel-interpolation", "insert-keyframe", "remove-keyframe", "set-keyframe-time", "set-keyframe-value"];
//#endregion 🔖️Mutation

//#region 🔖️DiffBuilders
/// 🧱️ Wraps a per-timeline `AnimTimelineDiff` into a full `SemioAnimationDiff` — the innermost
/// layer of the nested-diff tree every non-collection-root mutation ultimately builds on.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_timeline_field(index: usize, diff: AnimTimelineDiff) -> SemioAnimationDiff {
    SemioAnimationDiff { timelines: Some(IndexedTripleDiff { modified: vec![IndexModified { index, diff }], ..Default::default() }) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_channel_collection(timeline_index: usize, channels: IndexedTripleDiff<AnimChannelDiff, AnimChannel>) -> SemioAnimationDiff {
    diff_timeline_field(timeline_index, AnimTimelineDiff { name: None, channels: Some(channels) })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_channel_field(timeline_index: usize, index: usize, diff: AnimChannelDiff) -> SemioAnimationDiff {
    diff_channel_collection(timeline_index, IndexedTripleDiff { modified: vec![IndexModified { index, diff }], ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_keyframe_collection(timeline_index: usize, channel_index: usize, keyframes: IndexedTripleDiff<AnimKeyframeDiff, AnimKeyframe>) -> SemioAnimationDiff {
    diff_channel_field(timeline_index, channel_index, AnimChannelDiff { target: None, interpolation: None, keyframes: Some(keyframes) })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_keyframe_field(timeline_index: usize, channel_index: usize, index: usize, diff: AnimKeyframeDiff) -> SemioAnimationDiff {
    diff_keyframe_collection(timeline_index, channel_index, IndexedTripleDiff { modified: vec![IndexModified { index, diff }], ..Default::default() })
}
//#endregion 🔖️DiffBuilders

//#region 🔖️BaseAccessors
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn timeline_at(base: &SemioAnimationSnapshot, i: usize) -> Option<&AnimTimeline> {
    base.timelines.get(i)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn channel_at(base: &SemioAnimationSnapshot, ti: usize, ci: usize) -> Option<&AnimChannel> {
    base.timelines.get(ti).and_then(|t| t.channels.get(ci))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn keyframe_at(base: &SemioAnimationSnapshot, ti: usize, ci: usize, ki: usize) -> Option<&AnimKeyframe> {
    base.timelines.get(ti).and_then(|t| t.channels.get(ci)).and_then(|c| c.keyframes.get(ki))
}
//#endregion 🔖️BaseAccessors

/// 🧮️ Pure diff face of [`Mutation::diff`], named only in this subset's own reachable types (`protocol` is a private
/// `extern crate` alias, so an owner-root test adapter cannot bring the `Mutation` trait into scope).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_semio_animation_mutation(mutation: &SemioAnimationMutation, base: &SemioAnimationSnapshot) -> protocol::MutationOutcome<SemioAnimationDiff> {
    <SemioAnimationMutation as protocol::Mutation<SemioAnimationSnapshot>>::diff(mutation, base)
}


/// ↩️ Free-function face of [`SemioAnimationMutation`]'s own `protocol::Mutation::inverse`. `Mutation` is
/// declared by the os-kernel, which is an INTERNAL dependency of this plugin (aliased `protocol` in
/// `🦀️.rs`) and is therefore not nameable by a consumer that links only this crate — a
/// generated test host being the concrete case. Paired with `diff_semio_*_mutation` it makes the
/// undo law reachable without importing a trait the caller cannot name.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_semio_animation_mutation(mutation: &SemioAnimationMutation, base: &SemioAnimationSnapshot) -> Result<Vec<SemioAnimationMutation>, semio_framework_value::ValueError> {
    Ok({
    <SemioAnimationMutation as Mutation<SemioAnimationSnapshot>>::inverse(mutation, base)?

    })
}






//#endregion 🔖️MutationTrait

//#region SnapshotLit


//#endregion SnapshotLit

//#region OpCodecs










//#endregion OpCodecs

/// 🧱️ Module-scope (not `mod tests`-local) fixture + demo mutation cases — so the `🎹️composer`
/// conformance-law tests can reuse them, same promotion pattern every prior semio wave's report
/// documents (a private item of a child `mod tests` isn't visible to a sibling module).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn fixture() -> SemioAnimationSnapshot {
    SemioAnimationSnapshot {
        timelines: vec![AnimTimeline {
            name: Some("walk".into()),
            channels: vec![AnimChannel {
                target: AnimTarget { node: "hip".into(), property: crate::standards::v1::subsets::animation::schema::snapshot::AnimTargetProperty::Translation },
                interpolation: AnimInterpolation::Linear,
                keyframes: vec![AnimKeyframe { t: 0.0, value: AnimValue::Scalar { value: 1.0 } }],
            }],
        }],
        ..SemioAnimationSnapshot::default()
    }
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<SemioAnimationMutation> {
    let base = fixture();
    use SemioAnimationMutation::*;
    vec![
        InsertTimeline(insert_timeline::InsertTimeline { index: 1, timeline: AnimTimeline { name: Some("wave".into()), channels: vec![] } }),
        RemoveTimeline(remove_timeline::RemoveTimeline { index: 0 }),
        SetTimelineName(set_timeline_name::SetTimelineName { index: 0, name: None }),
        InsertChannel(insert_channel::InsertChannel { timeline_index: 0, index: 1, channel: base.timelines[0].channels[0].clone() }),
        RemoveChannel(remove_channel::RemoveChannel { timeline_index: 0, index: 0 }),
        SetChannelTarget(set_channel_target::SetChannelTarget { timeline_index: 0, index: 0, target: AnimTarget { node: "spine".into(), property: crate::standards::v1::subsets::animation::schema::snapshot::AnimTargetProperty::Rotation } }),
        SetChannelInterpolation(set_channel_interpolation::SetChannelInterpolation { timeline_index: 0, index: 0, interpolation: AnimInterpolation::Step }),
        InsertKeyframe(insert_keyframe::InsertKeyframe { timeline_index: 0, channel_index: 0, index: 1, keyframe: AnimKeyframe { t: 2.0, value: AnimValue::Scalar { value: 5.0 } } }),
        RemoveKeyframe(remove_keyframe::RemoveKeyframe { timeline_index: 0, channel_index: 0, index: 0 }),
        SetKeyframeTime(set_keyframe_time::SetKeyframeTime { timeline_index: 0, channel_index: 0, index: 0, t: 3.5 }),
        SetKeyframeValue(set_keyframe_value::SetKeyframeValue { timeline_index: 0, channel_index: 0, index: 0, value: AnimValue::Weights { values: vec![0.1, 0.9] } }),
    ]
}

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests

#[cfg(test)]
use protocol::{OpBinary,OpText};
