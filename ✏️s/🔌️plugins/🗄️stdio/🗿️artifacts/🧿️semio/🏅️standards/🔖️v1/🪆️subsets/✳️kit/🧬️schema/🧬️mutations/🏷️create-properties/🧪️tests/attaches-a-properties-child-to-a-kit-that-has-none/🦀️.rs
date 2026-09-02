//! 🧪️ `create-properties` fixture — `attaches-a-properties-child-to-a-kit-that-has-none`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: a kit that ALREADY has a properties child is
//! FATAL `mutation.duplicate-id`; otherwise the diff sets
//! `properties = Some(Some(ArtifactChild::new(child_id, target)))`. Unlike `objects`/`models`,
//! `properties` is a SINGLE optional child slot, so its diff type is the tri-state
//! `Option<Option<..>>` — and because this case takes the SET arm, its inner value is a real
//! object and the committed diff survives a JSON round trip intact.

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
    serde_json::from_str(BEFORE).expect("create-properties before snapshot decodes")
}
fn expected_after() -> SemioKitSnapshot {
    serde_json::from_str(AFTER).expect("create-properties after snapshot decodes")
}
fn mutation() -> SemioKitMutation {
    serde_json::from_str(MUTATION).expect("create-properties mutation decodes")
}

/// ▶️ The properties slot goes from absent to a handle; the two child collections stay put.
#[semio_framework_async_macros::async_test]
async fn attaches_the_properties_handle_to_an_empty_slot() {
    let base = before();
    assert!(base.properties.is_none(), "the fixture's whole point is a kit whose properties slot is empty");
    let produced = mutation().diff(&base).diff().apply(&base).expect("create-properties applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "create-properties/attaches-a-properties-child-to-a-kit-that-has-none: applied state differs from the committed after-snapshot");
    let handle = produced.properties.as_ref().expect("the properties slot must be populated afterwards");
    assert_eq!(handle.child_id, "props-1", "the handle keeps the payload's own child id");
    assert_eq!(handle.target.dialect.subset, "value", "kit properties are a value-subset document");
    assert_eq!((produced.objects, produced.models), (base.objects, base.models), "the child COLLECTIONS are a different slot and must not move");
}

/// ↩️ With an empty base slot the undo is a single `delete-properties`.
#[semio_framework_async_macros::async_test]
async fn the_undo_delete_properties_clears_the_slot_again() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "creating into an EMPTY slot undoes as the matching delete, not as another create");
    assert!(matches!(undo[0], SemioKitMutation::DeleteProperties(_)), "the undo must be delete-properties");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward create-properties applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo delete-properties applies");
    }
    assert_eq!(current, base, "create-properties/attaches-a-properties-child-to-a-kit-that-has-none: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — the before-snapshot omits the `properties` key entirely (it is the one kit field with `skip_serializing_if`), while every other collection is always emitted, empty or not.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioKitSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-properties/attaches-a-properties-child-to-a-kit-that-has-none: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("create-properties mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("create-properties mutation reparses");
    assert_eq!(reencoded, original, "create-properties/attaches-a-properties-child-to-a-kit-that-has-none: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the properties slot is empty, so the FATAL mutation.duplicate-id branch must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "create-properties/attaches-a-properties-child-to-a-kit-that-has-none: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "attaching a properties child into an empty slot must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only the `properties` slot, and its value is a real handle object.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioKitMutation as Mutation<SemioKitSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-properties/attaches-a-properties-child-to-a-kit-that-has-none: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the slot this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioKitDiff = serde_json::from_str(DIFF).expect("committed create-properties diff decodes");
    assert!(matches!(decoded.properties, Some(Some(_))), "the properties slot must decode as Some(Some(handle)) — set, not cleared");
    assert!(decoded.types.is_none() && decoded.designs.is_none() && decoded.objects.is_none() && decoded.models.is_none() && decoded.representations.is_none(), "no other kit slot may appear in the diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-properties/attaches-a-properties-child-to-a-kit-that-has-none: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioKitDiff = serde_json::from_str(DIFF).expect("committed create-properties diff decodes");
    let produced = decoded.apply(&before()).expect("committed create-properties diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-properties/attaches-a-properties-child-to-a-kit-that-has-none: committed diff did not carry before to after");
}
