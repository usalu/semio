//! 🧪️ `rotate-object` fixture — `rotates-the-object-a-half-turn-about-z`.
//!
//! Transcribed from `../../🔺️diff/🦀️component.rs`: any non-finite quaternion component is FATAL
//! `mutation.invariant`, an unchanged rotation is Warning `mutation.no-op`, and otherwise the BASE
//! transform is cloned with ONLY `rotation` overwritten. A half turn about Z — `(0, 0, 1, 0)` — is
//! the rotation chosen here precisely because every component is exactly representable, so the
//! canonical-JSON assertion holds without any float slack.

use crate::artifacts::semio::standards::v1::subsets::object::schema::diff::SemioObjectDiff;
use crate::artifacts::semio::standards::v1::subsets::object::schema::mutations::SemioObjectMutation;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> SemioObjectSnapshot {
    serde_json::from_str(BEFORE).expect("rotate-object before snapshot decodes")
}
fn expected_after() -> SemioObjectSnapshot {
    serde_json::from_str(AFTER).expect("rotate-object after snapshot decodes")
}
fn rotate_object() -> SemioObjectMutation {
    serde_json::from_str(MUTATION).expect("rotate-object mutation decodes")
}

/// ▶️ The identity rotation is replaced by the half turn; translation and scale are untouched.
#[semio_framework_async_macros::async_test]
async fn replaces_the_rotation_and_keeps_translation_and_scale() {
    let base = before();
    let produced = rotate_object().diff(&base).diff().apply(&base).expect("rotate-object applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "rotate-object/rotates-the-object-a-half-turn-about-z: applied state differs from the committed after-snapshot");
    assert_eq!((produced.transform.rotation.x, produced.transform.rotation.y), (0.0, 0.0), "a Z-axis rotation leaves the X/Y quaternion components at zero");
    assert_eq!((produced.transform.rotation.z, produced.transform.rotation.w), (1.0, 0.0), "the half turn about Z is the quaternion (0, 0, 1, 0)");
    assert_eq!(produced.transform.translation, base.transform.translation, "rotate-object must not touch the translation");
    assert_eq!(produced.transform.scale, base.transform.scale, "rotate-object must not touch the scale");
}

/// ↩️ The undo is a `rotate-object` back to BASE's own rotation.
#[semio_framework_async_macros::async_test]
async fn the_undo_rotate_object_restores_the_identity_rotation() {
    let base = before();
    let mutation = rotate_object();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "rotate-object undoes as exactly one rotate-object");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward rotate-object applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo rotate-object applies to the rotated object");
    }
    assert_eq!(current, base, "rotate-object/rotates-the-object-a-half-turn-about-z: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"RotateObject":{"rotation":{…}}}` payload are canonical — the
/// quaternion is a NAMED four-field struct, never a bare array.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioObjectSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "rotate-object/rotates-the-object-a-half-turn-about-z: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(rotate_object()).expect("rotate-object mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("rotate-object mutation reparses");
    assert_eq!(reencoded, original, "rotate-object/rotates-the-object-a-half-turn-about-z: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the quaternion is finite and differs from the base rotation.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_with_neither_invariant_nor_no_op() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "rotate-object/rotates-the-object-a-half-turn-about-z: this case is declared applied");
    let produced = rotate_object().diff(&before());
    assert!(produced.messages().is_empty(), "a finite, genuinely-different rotation must raise no diagnostics");
}

/// 🔺️ The produced delta equals the committed diff.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioObjectMutation as Mutation<SemioObjectSnapshot>>::diff(&rotate_object(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "rotate-object/rotates-the-object-a-half-turn-about-z: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical, carries the WHOLE transform (not just the rotation), and
/// touches no child slot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_carries_the_whole_transform() {
    let decoded: SemioObjectDiff = serde_json::from_str(DIFF).expect("committed rotate-object diff decodes");
    let transform = decoded.transform.as_ref().expect("rotate-object must write the transform slot");
    assert_eq!(transform.scale, before().transform.scale, "the diff carries the whole transform, so the untouched scale must be the base scale");
    assert!(decoded.brep.is_none() && decoded.mesh.is_none() && decoded.properties.is_none(), "rotate-object must leave all three child slots untouched");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "rotate-object/rotates-the-object-a-half-turn-about-z: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioObjectDiff = serde_json::from_str(DIFF).expect("committed rotate-object diff decodes");
    let produced = decoded.apply(&before()).expect("committed rotate-object diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "rotate-object/rotates-the-object-a-half-turn-about-z: committed diff did not carry before to after");
}
