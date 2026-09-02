//! 🧪️ `delete-model` fixture — `detaches-the-only-model-child-and-keeps-the-object-child`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: an unknown `child_id` is Error
//! `mutation.target-missing`; otherwise `models` is rebuilt by filtering that handle out. The
//! mirror of `delete-object`: the sibling `objects` collection is deliberately untouched, which is
//! what this fixture's two-collection base is for.

use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::SemioKitDiff;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::SemioKitMutation;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> SemioKitSnapshot {
    serde_json::from_str(BEFORE).expect("delete-model before snapshot decodes")
}
fn expected_after() -> SemioKitSnapshot {
    serde_json::from_str(AFTER).expect("delete-model after snapshot decodes")
}
fn mutation() -> SemioKitMutation {
    serde_json::from_str(MUTATION).expect("delete-model mutation decodes")
}

/// ▶️ The model handle goes; the object handle stays.
#[semio_framework_async_macros::async_test]
async fn detaches_the_model_handle_and_leaves_the_object_handle() {
    let base = before();
    assert!(!base.objects.is_empty(), "the fixture needs a sibling object child for the claim to mean anything");
    let produced = mutation().diff(&base).diff().apply(&base).expect("delete-model applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "delete-model/detaches-the-only-model-child-and-keeps-the-object-child: applied state differs from the committed after-snapshot");
    assert!(produced.models.is_empty(), "the only model handle must be gone");
    assert_eq!(produced.objects, base.objects, "the parallel object-child collection must survive untouched");
}

/// ↩️ The undo re-attaches the handle with its captured target.
#[semio_framework_async_macros::async_test]
async fn the_undo_create_model_reattaches_the_captured_handle() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "delete-model of an existing handle undoes as exactly one create-model");
    let SemioKitMutation::CreateModel(recreate) = &undo[0] else { panic!("delete-model must undo as create-model") };
    assert_eq!(recreate.target, base.models[0].target, "the undo must recapture the detached handle's own target ref");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward delete-model applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo create-model applies");
    }
    assert_eq!(current, base, "delete-model/detaches-the-only-model-child-and-keeps-the-object-child: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the `{"DeleteModel":{"child_id":"mod-1"}}` payload are canonical fixed points.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioKitSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-model/detaches-the-only-model-child-and-keeps-the-object-child: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("delete-model mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("delete-model mutation reparses");
    assert_eq!(reencoded, original, "delete-model/detaches-the-only-model-child-and-keeps-the-object-child: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the model child exists, so mutation.target-missing must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-model/detaches-the-only-model-child-and-keeps-the-object-child: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "detaching an existing model child must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only the `models` slot, carrying the emptied list.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioKitMutation as Mutation<SemioKitSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-model/detaches-the-only-model-child-and-keeps-the-object-child: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the slot this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioKitDiff = serde_json::from_str(DIFF).expect("committed delete-model diff decodes");
    assert_eq!(decoded.models.as_ref().map(|list| list.values.len()), Some(0), "the diff carries the emptied model-child list");
    assert!(decoded.types.is_none() && decoded.designs.is_none() && decoded.objects.is_none() && decoded.properties.is_none() && decoded.representations.is_none(), "no other kit slot may appear in the diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-model/detaches-the-only-model-child-and-keeps-the-object-child: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioKitDiff = serde_json::from_str(DIFF).expect("committed delete-model diff decodes");
    let produced = decoded.apply(&before()).expect("committed delete-model diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-model/detaches-the-only-model-child-and-keeps-the-object-child: committed diff did not carry before to after");
}
