//! 🧪️ `📄set-snapshot` fixture — `steps-the-spin-channel-and-appends-a-keyframe`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! 🎬️ The case switches the `Spin` timeline's ROTATION channel from `Linear` to `Step` sampling
//! and appends a third quaternion keyframe at t=2. The timeline's `name`, the channel's `target`,
//! its two existing keyframes and the sibling SCALE channel are all unchanged — so
//! `SemioAnimationDiff` must nest three index-keyed triples (`timelines[0] → channels[0] →
//! keyframes.added[2]`) and set nothing else.
//!
//! 🪆️ `AnimTimelineDiff::name` is a tri-state `Option<Option<String>>`; "clear the timeline name"
//! is `Some(None)`, which serde writes as bare `null` and reads back as `None` (= unchanged), so no
//! committed fixture may express it. Keeping the name stable here leaves the slot absent.

use crate::artifacts::semio::standards::v1::subsets::animation::schema::diff::SemioAnimationDiff;
use crate::artifacts::semio::standards::v1::subsets::animation::schema::mutations::{apply_semio_animation_mutation, SemioAnimationMutation};
use crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::{AnimInterpolation, AnimValue, SemioAnimationSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> SemioAnimationSnapshot {
    serde_json::from_str(BEFORE).expect("before animation snapshot decodes")
}
fn expected_after() -> SemioAnimationSnapshot {
    serde_json::from_str(AFTER).expect("after animation snapshot decodes")
}
fn mutation() -> SemioAnimationMutation {
    serde_json::from_str(MUTATION).expect("set-snapshot mutation decodes")
}

/// ▶️ `set-snapshot` carries the animation to exactly the committed `after`: a stepped rotation
/// channel with three keyframes.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_semio_animation_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "semio-animation/set-snapshot: a genuinely changed animation must not raise any message");
    let rotation = &snapshot.timelines[0].channels[0];
    assert_eq!(rotation.interpolation, AnimInterpolation::Step, "semio-animation/set-snapshot: the rotation channel must resample as Step");
    assert_eq!(rotation.keyframes.len(), 3, "semio-animation/set-snapshot: the third keyframe must be appended");
    assert!(matches!(rotation.keyframes[2].value, AnimValue::Quat { .. }), "semio-animation/set-snapshot: a rotation keyframe carries a named quaternion, never a bare four-element array");
    assert_eq!(snapshot.timelines[0].channels[1], before().timelines[0].channels[1], "semio-animation/set-snapshot: the scale channel must be carried over untouched");
    assert_eq!(snapshot, expected_after(), "semio-animation/set-snapshot: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse of `set-snapshot` is `set-snapshot(base)` — it must drop the appended keyframe
/// and return the channel to Linear sampling.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <SemioAnimationMutation as protocol::Mutation<SemioAnimationSnapshot>>::inverse(&mutation, &base);
    let mut snapshot = base.clone();
    apply_semio_animation_mutation(&mut snapshot, &mutation);
    for step in &inverse {
        apply_semio_animation_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, base, "semio-animation/set-snapshot: inverse did not restore the committed before-snapshot");
}

/// 🔣️ Both committed animations and the mutation are already canonical: `AnimValue` and
/// `AnimTargetProperty` are internally tagged on `kind`, and a timeline's optional `name` is
/// ALWAYS written (its field carries `#[serde(default)]` but no `skip_serializing_if`).
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioAnimationSnapshot = serde_json::from_str(text).expect("animation snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("animation snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("animation snapshot reparses");
        assert_eq!(reencoded, original, "semio-animation/set-snapshot: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("set-snapshot mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("set-snapshot mutation reparses");
    assert_eq!(reencoded, original, "semio-animation/set-snapshot: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is `applied` — the animation really moves, so the `mutation.no-op`
/// warning an identical set-snapshot would raise never appears.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "semio-animation/set-snapshot: this fixture declares an applied outcome");
    let mut snapshot = before();
    let produced = apply_semio_animation_mutation(&mut snapshot, &mutation());
    assert!(produced.messages().is_empty(), "semio-animation/set-snapshot: declared applied, so no diagnostic may be raised");
    assert_ne!(snapshot, before(), "semio-animation/set-snapshot: an applied set-snapshot must actually move the animation");
}

/// 🔺️ The sparse `SemioAnimationDiff` this mutation produces is exactly the committed diff — the
/// load-bearing assertion: the scale channel must not appear, the two surviving keyframes must not
/// be re-listed as `modified`, and the timeline's tri-state `name` slot must stay absent.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioAnimationMutation as protocol::Mutation<SemioAnimationSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced animation diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed animation diff decodes");
    assert_eq!(produced, committed, "semio-animation/set-snapshot: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to `SemioAnimationDiff`: a pure append at
/// the keyframe level, whose `index` refers to the FINAL position.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: SemioAnimationDiff = serde_json::from_str(DIFF).expect("committed animation diff decodes");
    let timelines = decoded.timelines.as_ref().expect("the committed diff carries a timelines triple");
    assert!(timelines.removed.is_empty() && timelines.added.is_empty() && timelines.modified.len() == 1, "semio-animation/set-snapshot: the single timeline must be patched in place");
    assert!(timelines.modified[0].diff.name.is_none(), "semio-animation/set-snapshot: the tri-state timeline name must stay absent, not a round-trip-lossy null");
    let channels = timelines.modified[0].diff.channels.as_ref().expect("the patched timeline carries a channels triple");
    assert!(channels.removed.is_empty() && channels.added.is_empty() && channels.modified.len() == 1 && channels.modified[0].index == 0, "semio-animation/set-snapshot: exactly the rotation channel may be patched");
    let keyframes = channels.modified[0].diff.keyframes.as_ref().expect("the patched channel carries a keyframes triple");
    assert!(keyframes.removed.is_empty() && keyframes.modified.is_empty() && keyframes.added.len() == 1 && keyframes.added[0].index == 2, "semio-animation/set-snapshot: appending a keyframe must neither remove nor patch an existing one");
    assert!(channels.modified[0].diff.target.is_none(), "semio-animation/set-snapshot: the channel's animated target did not move and must stay absent");
    let reencoded = serde_json::to_value(&decoded).expect("animation diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed animation diff reparses");
    assert_eq!(reencoded, original, "semio-animation/set-snapshot: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to `before` yields the committed `after` — the
/// interpolation switch plus the appended keyframe is a complete description of the change.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioAnimationDiff = serde_json::from_str(DIFF).expect("committed animation diff decodes");
    let produced = <SemioAnimationDiff as protocol::MutationDiff<SemioAnimationSnapshot>>::apply(&decoded, &before()).expect("committed animation diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "semio-animation/set-snapshot: committed diff did not carry before to after");
}
