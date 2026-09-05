//! 🧪️ `create-object` fixture — `🪄️attaches-a-second-object-child`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: a duplicate `child_id` is FATAL
//! `mutation.duplicate-id`; otherwise a new `store::ArtifactChild` handle is PUSHED onto `objects`.
//! `objects` is a child COLLECTION (unlike `✳️object`'s single-slot `brep`/`mesh`), so a kit can
//! hold many object children and the diff rebuilds the whole list.

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
    serde_json::from_str(BEFORE).expect("create-object before snapshot decodes")
}
fn expected_after() -> SemioKitSnapshot {
    serde_json::from_str(AFTER).expect("create-object after snapshot decodes")
}
fn mutation() -> SemioKitMutation {
    serde_json::from_str(MUTATION).expect("create-object mutation decodes")
}

/// ▶️ A second object handle appears; the model children and everything else stay put.
#[semio_framework_async_macros::async_test]
async fn attaches_the_second_object_handle_without_touching_the_models() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("create-object applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "create-object/attaches-a-second-object-child: applied state differs from the committed after-snapshot");
    assert_eq!(produced.objects.len(), base.objects.len() + 1, "create-object adds exactly one object child");
    let created = produced.objects.last().expect("the new handle is pushed at the end");
    assert_eq!(created.child_id, "obj-2", "the handle keeps the payload's own child id");
    assert_eq!(created.target.dialect.subset, "object", "the handle points at an object-subset artifact");
    assert_eq!(produced.models, base.models, "the parallel model-child collection must not move");
}

/// ↩️ `create-object`'s undo is a single `delete-object` for the same child id.
#[semio_framework_async_macros::async_test]
async fn the_undo_delete_object_detaches_the_second_handle_again() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "create-object undoes as exactly one delete-object");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward create-object applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo delete-object applies");
    }
    assert_eq!(current, base, "create-object/attaches-a-second-object-child: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — a child handle encodes as `{"childId":…,"target":{"artifactId":…,"dialect":{…}}}` while the payload key stays `child_id`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioKitSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-object/attaches-a-second-object-child: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("create-object mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("create-object mutation reparses");
    assert_eq!(reencoded, original, "create-object/attaches-a-second-object-child: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: no object child with id obj-2 exists, so the FATAL mutation.duplicate-id branch must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "create-object/attaches-a-second-object-child: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "attaching an object child with a fresh id must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only the `objects` slot.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioKitMutation as Mutation<SemioKitSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-object/attaches-a-second-object-child: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the slot this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioKitDiff = serde_json::from_str(DIFF).expect("committed create-object diff decodes");
    assert_eq!(decoded.objects.as_ref().map(|list| list.values.len()), Some(2), "the diff carries the whole rebuilt object-child list");
    assert!(decoded.types.is_none() && decoded.designs.is_none() && decoded.models.is_none() && decoded.properties.is_none() && decoded.representations.is_none(), "no other kit slot may appear in the diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-object/attaches-a-second-object-child: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioKitDiff = serde_json::from_str(DIFF).expect("committed create-object diff decodes");
    let produced = decoded.apply(&before()).expect("committed create-object diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-object/attaches-a-second-object-child: committed diff did not carry before to after");
}
