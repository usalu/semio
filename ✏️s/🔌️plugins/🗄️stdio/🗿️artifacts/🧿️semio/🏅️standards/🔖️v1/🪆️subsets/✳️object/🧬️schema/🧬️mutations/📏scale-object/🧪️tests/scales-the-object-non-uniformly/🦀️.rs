//! 🧪️ `scale-object` fixture — `scales-the-object-non-uniformly`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`, whose invariant guard is STRICTER than
//! `move-object`'s: a scale component must be finite AND strictly positive, otherwise FATAL
//! `mutation.invariant`. `(2, 0.5, 4)` is deliberately non-uniform and all-positive — it exercises
//! the real path while every component stays exactly representable.

use crate::artifacts::semio::standards::v1::subsets::object::schema::diff::SemioObjectDiff;
use crate::artifacts::semio::standards::v1::subsets::object::schema::mutations::SemioObjectMutation;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> SemioObjectSnapshot {
    serde_json::from_str(BEFORE).expect("scale-object before snapshot decodes")
}
fn expected_after() -> SemioObjectSnapshot {
    serde_json::from_str(AFTER).expect("scale-object after snapshot decodes")
}
fn scale_object() -> SemioObjectMutation {
    serde_json::from_str(MUTATION).expect("scale-object mutation decodes")
}

/// ▶️ Unit scale becomes the non-uniform `(2, 0.5, 4)`; translation and rotation stay put.
#[semio_framework_async_macros::async_test]
async fn replaces_the_scale_with_a_non_uniform_one() {
    let base = before();
    let produced = scale_object().diff(&base).diff().apply(&base).expect("scale-object applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "scale-object/scales-the-object-non-uniformly: applied state differs from the committed after-snapshot");
    assert_eq!((produced.transform.scale.x, produced.transform.scale.y, produced.transform.scale.z), (2.0, 0.5, 4.0), "the scale must become the payload's absolute per-axis values");
    assert!(produced.transform.scale.x != produced.transform.scale.y, "this case is deliberately NON-uniform — a uniform scale would not exercise per-axis handling");
    assert_eq!(produced.transform.translation, base.transform.translation, "scale-object must not touch the translation");
    assert_eq!(produced.transform.rotation, base.transform.rotation, "scale-object must not touch the rotation");
}

/// ↩️ The undo is a `scale-object` back to BASE's own (unit) scale.
#[semio_framework_async_macros::async_test]
async fn the_undo_scale_object_restores_the_unit_scale() {
    let base = before();
    let mutation = scale_object();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "scale-object undoes as exactly one scale-object");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward scale-object applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo scale-object applies to the scaled object");
    }
    assert_eq!(current, base, "scale-object/scales-the-object-non-uniformly: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"ScaleObject":{"scale":{…}}}` payload are canonical.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioObjectSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "scale-object/scales-the-object-non-uniformly: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(scale_object()).expect("scale-object mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("scale-object mutation reparses");
    assert_eq!(reencoded, original, "scale-object/scales-the-object-non-uniformly: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: every component is finite AND strictly positive, so the stricter
/// `mutation.invariant` guard must not fire.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_with_a_strictly_positive_scale() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "scale-object/scales-the-object-non-uniformly: this case is declared applied");
    let produced = scale_object().diff(&before());
    assert!(produced.messages().is_empty(), "a finite, strictly-positive, genuinely-different scale must raise no diagnostics");
}

/// 🔺️ The produced delta equals the committed diff.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioObjectMutation as Mutation<SemioObjectSnapshot>>::diff(&scale_object(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "scale-object/scales-the-object-non-uniformly: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and touches only the `transform` slot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_touches_only_the_transform_slot() {
    let decoded: SemioObjectDiff = serde_json::from_str(DIFF).expect("committed scale-object diff decodes");
    let transform = decoded.transform.as_ref().expect("scale-object must write the transform slot");
    assert_eq!(transform.translation, before().transform.translation, "the diff carries the whole transform, so the untouched translation must be the base translation");
    assert!(decoded.brep.is_none() && decoded.mesh.is_none() && decoded.properties.is_none(), "scale-object must leave all three child slots untouched");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "scale-object/scales-the-object-non-uniformly: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioObjectDiff = serde_json::from_str(DIFF).expect("committed scale-object diff decodes");
    let produced = decoded.apply(&before()).expect("committed scale-object diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "scale-object/scales-the-object-non-uniformly: committed diff did not carry before to after");
}
