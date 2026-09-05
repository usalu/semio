//! 🧪️ `move-object` fixture — `🚚️moves-the-object-to-a-new-translation`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: a non-finite component is FATAL
//! `mutation.invariant`, an unchanged translation is Warning `mutation.no-op`, and otherwise the
//! BASE transform is cloned and ONLY its `translation` overwritten. `SemioObjectDiff` has four
//! independent `Option` slots, so the committed diff must populate `transform` and leave
//! `brep`/`mesh`/`properties` absent entirely.

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
    serde_json::from_str(BEFORE).expect("move-object before snapshot decodes")
}
fn expected_after() -> SemioObjectSnapshot {
    serde_json::from_str(AFTER).expect("move-object after snapshot decodes")
}
fn move_object() -> SemioObjectMutation {
    serde_json::from_str(MUTATION).expect("move-object mutation decodes")
}

/// ▶️ Translation is replaced outright (it is an absolute placement, not a delta); rotation and
/// scale ride along untouched.
#[semio_framework_async_macros::async_test]
async fn replaces_the_translation_and_keeps_rotation_and_scale() {
    let base = before();
    let produced = move_object().diff(&base).diff().apply(&base).expect("move-object applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "move-object/moves-the-object-to-a-new-translation: applied state differs from the committed after-snapshot");
    assert_eq!(produced.transform.translation.x, 2.0, "translation.x must become the payload's absolute value, not base + payload");
    assert_eq!(produced.transform.translation.y, -0.5, "translation.y must become the payload's absolute value");
    assert_eq!(produced.transform.translation.z, 4.0, "translation.z must become the payload's absolute value");
    assert_eq!(produced.transform.rotation, base.transform.rotation, "move-object must not touch the rotation");
    assert_eq!(produced.transform.scale, base.transform.scale, "move-object must not touch the scale");
    assert_eq!((produced.brep.is_none(), produced.mesh.is_none(), produced.properties.is_none()), (true, true, true), "move-object must not touch any child slot");
}

/// ↩️ The undo is a `move-object` back to BASE's own translation.
#[semio_framework_async_macros::async_test]
async fn the_undo_move_object_restores_the_original_translation() {
    let base = before();
    let mutation = move_object();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "move-object undoes as exactly one move-object");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward move-object applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo move-object applies to the moved object");
    }
    assert_eq!(current, base, "move-object/moves-the-object-to-a-new-translation: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"MoveObject":{"translation":{…}}}` payload are canonical — every
/// coordinate is a dyadic `f64` so decode→encode is exact.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioObjectSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "move-object/moves-the-object-to-a-new-translation: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(move_object()).expect("move-object mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("move-object mutation reparses");
    assert_eq!(reencoded, original, "move-object/moves-the-object-to-a-new-translation: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: every component is finite and differs from the base translation, so
/// neither the FATAL `mutation.invariant` nor the `mutation.no-op` warning may fire.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_with_neither_invariant_nor_no_op() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "move-object/moves-the-object-to-a-new-translation: this case is declared applied");
    let produced = move_object().diff(&before());
    assert!(produced.messages().is_empty(), "a finite, genuinely-different translation must raise no diagnostics");
}

/// 🔺️ The produced delta equals the committed diff.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioObjectMutation as Mutation<SemioObjectSnapshot>>::diff(&move_object(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "move-object/moves-the-object-to-a-new-translation: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and touches ONLY the `transform` slot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_touches_only_the_transform_slot() {
    let decoded: SemioObjectDiff = serde_json::from_str(DIFF).expect("committed move-object diff decodes");
    assert!(decoded.transform.is_some(), "move-object must write the transform slot");
    assert!(decoded.brep.is_none() && decoded.mesh.is_none() && decoded.properties.is_none(), "move-object must leave all three child slots untouched");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(committed.as_object().map(|map| map.len()), Some(1), "the committed diff JSON must carry exactly the transform key");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    assert_eq!(reencoded, committed, "move-object/moves-the-object-to-a-new-translation: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioObjectDiff = serde_json::from_str(DIFF).expect("committed move-object diff decodes");
    let produced = decoded.apply(&before()).expect("committed move-object diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "move-object/moves-the-object-to-a-new-translation: committed diff did not carry before to after");
}
