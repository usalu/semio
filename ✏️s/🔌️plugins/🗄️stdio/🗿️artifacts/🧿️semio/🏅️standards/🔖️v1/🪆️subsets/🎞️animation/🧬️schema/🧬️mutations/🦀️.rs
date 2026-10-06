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

use crate::standards::v1::subsets::animation::schema::diff::{diff_set_snapshot, AnimChannelDiff, AnimKeyframeDiff, AnimTimelineDiff, SemioAnimationDiff};
use crate::standards::v1::subsets::animation::schema::snapshot::{AnimChannel, AnimInterpolation, AnimKeyframe, AnimTarget, AnimTimeline, AnimValue, SemioAnimationSnapshot};
use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff};
use protocol::Mutation;
/// 🔧️ `MutationDiff` added — the `#[cfg(test)] mod tests` block below calls `diff.apply(&base)`
/// via method syntax on `SemioAnimationDiff`, which needs `MutationDiff` in scope (W2b closer fix).
#[cfg(test)]
use protocol::MutationDiff;
use protocol::{OpBinary, OpText};

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
#[path = "🩹️patch-snapshot/🦀️.rs"]
pub mod patch_snapshot;
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🏷️set-timeline-name/🦀️.rs"]
pub mod set_timeline_name;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = SemioAnimationSnapshot, diff = SemioAnimationDiff, schema = "SemioAnimationMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum SemioAnimationMutation {
    /// 📦️ Full-snapshot replace, still sparse under the hood (`diff_set_snapshot` = `between`).
    SetSnapshot(set_snapshot::SetSnapshot),
    PatchSnapshot(patch_snapshot::PatchSnapshot),
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
    &["set-snapshot", "insert-timeline", "remove-timeline", "set-timeline-name", "insert-channel", "remove-channel", "set-channel-target", "set-channel-interpolation", "insert-keyframe", "remove-keyframe", "set-keyframe-time", "set-keyframe-value", "patch-snapshot"];
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

/// ▶️ Applies a mutation to `snapshot` in place, returning the diff (mirrors gif's
/// `apply_gif_mutation` convention — used by the builder's `mutate()` and the set-snapshot leaf).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_semio_animation_mutation(snapshot: &mut SemioAnimationSnapshot, mutation: &SemioAnimationMutation) -> protocol::MutationOutcome<SemioAnimationDiff> {
    let outcome = <SemioAnimationMutation as Mutation<SemioAnimationSnapshot>>::diff(mutation, snapshot);
    outcome.apply_to(snapshot)
}

/// ↩️ Free-function face of [`SemioAnimationMutation`]'s own `protocol::Mutation::inverse`. `Mutation` is
/// declared by the os-kernel, which is an INTERNAL dependency of this plugin (aliased `protocol` in
/// `🦀️.rs`) and is therefore not nameable by a consumer that links only this crate — a
/// generated test host being the concrete case. Paired with [`apply_semio_animation_mutation`] it makes the
/// undo law reachable without importing a trait the caller cannot name.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_semio_animation_mutation(mutation: &SemioAnimationMutation, base: &SemioAnimationSnapshot) -> Result<Vec<SemioAnimationMutation>, semio_framework_value::ValueError> {
    Ok({
    <SemioAnimationMutation as Mutation<SemioAnimationSnapshot>>::inverse(mutation, base)?

    })
}



//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &SemioAnimationMutation, base: &SemioAnimationSnapshot) -> protocol::MutationOutcome<SemioAnimationDiff> {
    use SemioAnimationMutation::*;
    protocol::MutationOutcome::new(match this {
        PatchSnapshot(payload) => return <patch_snapshot::PatchSnapshot as protocol::MutationKind<SemioAnimationSnapshot, SemioAnimationMutation>>::diff(payload, base),
        SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff_set_snapshot(base, snapshot),
        InsertTimeline(insert_timeline::InsertTimeline { index, timeline }) => SemioAnimationDiff { timelines: Some(IndexedTripleDiff { added: vec![IndexAdded { index: *index, item: timeline.clone() }], ..Default::default() }) },
        RemoveTimeline(remove_timeline::RemoveTimeline { index }) => SemioAnimationDiff { timelines: Some(IndexedTripleDiff { removed: vec![*index], ..Default::default() }) },
        SetTimelineName(set_timeline_name::SetTimelineName { index, name }) => diff_timeline_field(*index, AnimTimelineDiff { name: Some(name.clone()), channels: None }),
        InsertChannel(insert_channel::InsertChannel { timeline_index, index, channel }) => diff_channel_collection(*timeline_index, IndexedTripleDiff { added: vec![IndexAdded { index: *index, item: channel.clone() }], ..Default::default() }),
        RemoveChannel(remove_channel::RemoveChannel { timeline_index, index }) => diff_channel_collection(*timeline_index, IndexedTripleDiff { removed: vec![*index], ..Default::default() }),
        SetChannelTarget(set_channel_target::SetChannelTarget { timeline_index, index, target }) => diff_channel_field(*timeline_index, *index, AnimChannelDiff { target: Some(target.clone()), interpolation: None, keyframes: None }),
        SetChannelInterpolation(set_channel_interpolation::SetChannelInterpolation { timeline_index, index, interpolation }) => {
            diff_channel_field(*timeline_index, *index, AnimChannelDiff { target: None, interpolation: Some(*interpolation), keyframes: None })
        }
        InsertKeyframe(insert_keyframe::InsertKeyframe { timeline_index, channel_index, index, keyframe }) => {
            diff_keyframe_collection(*timeline_index, *channel_index, IndexedTripleDiff { added: vec![IndexAdded { index: *index, item: keyframe.clone() }], ..Default::default() })
        }
        RemoveKeyframe(remove_keyframe::RemoveKeyframe { timeline_index, channel_index, index }) => diff_keyframe_collection(*timeline_index, *channel_index, IndexedTripleDiff { removed: vec![*index], ..Default::default() }),
        SetKeyframeTime(set_keyframe_time::SetKeyframeTime { timeline_index, channel_index, index, t }) => diff_keyframe_field(*timeline_index, *channel_index, *index, AnimKeyframeDiff { t: Some(*t), value: None }),
        SetKeyframeValue(set_keyframe_value::SetKeyframeValue { timeline_index, channel_index, index, value }) => diff_keyframe_field(*timeline_index, *channel_index, *index, AnimKeyframeDiff { t: None, value: Some(value.clone()) }),
    })
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &SemioAnimationMutation, base: &SemioAnimationSnapshot) -> Result<Vec<SemioAnimationMutation>, semio_framework_value::ValueError> {
    Ok({
    use SemioAnimationMutation::*;
    match this {
        PatchSnapshot(payload) => <patch_snapshot::PatchSnapshot as protocol::MutationKind<SemioAnimationSnapshot, SemioAnimationMutation>>::inverse(payload, base)?,
        SetSnapshot(_) => vec![SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        InsertTimeline(insert_timeline::InsertTimeline { index, .. }) => vec![RemoveTimeline(remove_timeline::RemoveTimeline { index: *index })],
        RemoveTimeline(remove_timeline::RemoveTimeline { index }) => match timeline_at(base, *index) {
            Some(t) => vec![InsertTimeline(insert_timeline::InsertTimeline { index: *index, timeline: t.clone() })],
            None => Vec::new(),
        },
        SetTimelineName(set_timeline_name::SetTimelineName { index, .. }) => vec![SetTimelineName(set_timeline_name::SetTimelineName { index: *index, name: timeline_at(base, *index).and_then(|t| t.name.clone()) })],
        InsertChannel(insert_channel::InsertChannel { timeline_index, index, .. }) => vec![RemoveChannel(remove_channel::RemoveChannel { timeline_index: *timeline_index, index: *index })],
        RemoveChannel(remove_channel::RemoveChannel { timeline_index, index }) => match channel_at(base, *timeline_index, *index) {
            Some(c) => vec![InsertChannel(insert_channel::InsertChannel { timeline_index: *timeline_index, index: *index, channel: c.clone() })],
            None => Vec::new(),
        },
        SetChannelTarget(set_channel_target::SetChannelTarget { timeline_index, index, .. }) => match channel_at(base, *timeline_index, *index) {
            Some(c) => vec![SetChannelTarget(set_channel_target::SetChannelTarget { timeline_index: *timeline_index, index: *index, target: c.target.clone() })],
            None => Vec::new(),
        },
        SetChannelInterpolation(set_channel_interpolation::SetChannelInterpolation { timeline_index, index, .. }) => match channel_at(base, *timeline_index, *index) {
            Some(c) => vec![SetChannelInterpolation(set_channel_interpolation::SetChannelInterpolation { timeline_index: *timeline_index, index: *index, interpolation: c.interpolation })],
            None => Vec::new(),
        },
        InsertKeyframe(insert_keyframe::InsertKeyframe { timeline_index, channel_index, index, .. }) => {
            vec![RemoveKeyframe(remove_keyframe::RemoveKeyframe { timeline_index: *timeline_index, channel_index: *channel_index, index: *index })]
        }
        RemoveKeyframe(remove_keyframe::RemoveKeyframe { timeline_index, channel_index, index }) => match keyframe_at(base, *timeline_index, *channel_index, *index) {
            Some(k) => vec![InsertKeyframe(insert_keyframe::InsertKeyframe { timeline_index: *timeline_index, channel_index: *channel_index, index: *index, keyframe: k.clone() })],
            None => Vec::new(),
        },
        SetKeyframeTime(set_keyframe_time::SetKeyframeTime { timeline_index, channel_index, index, .. }) => match keyframe_at(base, *timeline_index, *channel_index, *index) {
            Some(k) => vec![SetKeyframeTime(set_keyframe_time::SetKeyframeTime { timeline_index: *timeline_index, channel_index: *channel_index, index: *index, t: k.t })],
            None => Vec::new(),
        },
        SetKeyframeValue(set_keyframe_value::SetKeyframeValue { timeline_index, channel_index, index, .. }) => match keyframe_at(base, *timeline_index, *channel_index, *index) {
            Some(k) => vec![SetKeyframeValue(set_keyframe_value::SetKeyframeValue { timeline_index: *timeline_index, channel_index: *channel_index, index: *index, value: k.value.clone() })],
            None => Vec::new(),
        },
    }

    })
}
//#endregion 🔖️MutationTrait

//#region SnapshotLit


//#endregion SnapshotLit

//#region OpCodecs




/// 🧾️ Each record kind's text-grammar tag, the head `decode_op` re-prefixes onto the argument tail before `parse_op`.
const TEXT_KEYWORDS: [(&str, &str); 12] = [
    ("set-snapshot", "S"),
    ("insert-timeline", "IT"),
    ("remove-timeline", "RT"),
    ("set-timeline-name", "TN"),
    ("insert-channel", "IC"),
    ("remove-channel", "RC"),
    ("set-channel-target", "CT"),
    ("set-channel-interpolation", "CI"),
    ("insert-keyframe", "IK"),
    ("remove-keyframe", "RK"),
    ("set-keyframe-time", "KT"),
    ("set-keyframe-value", "KV"),
];


const OP_BINARY_FORMAT: u8 = 1;


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
        SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
        PatchSnapshot(patch_snapshot::PatchSnapshot { patch: semio_s_artifact_stdio_contract::editing::SnapshotPatch::Set { path: "/schema".into(), value: semio_framework_value::DslValue::String("stdio.patch-snapshot.witness".into()) } }),
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

//#region 🧪️FixtureCases
/// 🧪️ Handcrafted `📸️set-snapshot` fixture cases, wired from this tree's own mutations root so
/// `🦀️.rs` stays untouched (`#[path]` on a non-inline module resolves against this file's own
/// directory).
#[cfg(test)]
#[path = "📸️set-snapshot/🧪️tests/🌀️steps/🦀️.rs"]
mod set_snapshot_steps_the_spin_channel_and_appends_a_keyframe;
//#endregion 🧪️FixtureCases
