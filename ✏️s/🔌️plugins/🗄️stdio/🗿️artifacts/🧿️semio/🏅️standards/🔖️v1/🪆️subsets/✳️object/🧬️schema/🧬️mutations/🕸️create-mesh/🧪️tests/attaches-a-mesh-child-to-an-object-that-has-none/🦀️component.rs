//! 🧪️ `🕸️create-mesh` fixture — `attaches-a-mesh-child-to-an-object-that-has-none`.
//!
//! Transcribed from `../../🔺️diff/🦀️component.rs`: an object that ALREADY has a mesh child is
//! FATAL `mutation.duplicate-id`; otherwise the diff sets `mesh = Some(Some(ArtifactChild::new(
//! child_id, target)))`. The outer `Some` means "this diff writes the slot", the inner `Some`
//! means "to this handle" — and because the inner value is a real object rather than `null`, this
//! case's committed diff survives a JSON round trip intact (unlike its `DeleteMesh` sibling).
//! A child handle is exactly two strings: `childId` plus the target `ArtifactRef`; the child's
//! own content never appears in the parent.

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
    serde_json::from_str(BEFORE).expect("🕸️create-mesh before snapshot decodes")
}
fn expected_after() -> SemioObjectSnapshot {
    serde_json::from_str(AFTER).expect("🕸️create-mesh after snapshot decodes")
}
fn mutation() -> SemioObjectMutation {
    serde_json::from_str(MUTATION).expect("🕸️create-mesh mutation decodes")
}

/// ▶️ The mesh slot goes from empty to a handle; the transform and the other two slots stay empty.
#[semio_framework_async_macros::async_test]
async fn attaches_the_mesh_handle_to_an_object_that_had_none() {
    let base = before();
    assert!(base.mesh.is_none(), "the fixture's whole point is a base whose mesh slot is empty");
    let produced = mutation().diff(&base).diff().apply(&base).expect("🕸️create-mesh applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "🕸️create-mesh/attaches-a-mesh-child-to-an-object-that-has-none: applied state differs from the committed after-snapshot");
    let handle = produced.mesh.as_ref().expect("the mesh slot must be populated afterwards");
    assert_eq!(handle.child_id, "mesh-1", "the handle keeps the payload's own child id");
    assert_eq!(handle.target.dialect.subset, "mesh", "the handle points at a mesh-subset artifact");
    assert_eq!(produced.transform, base.transform, "🕸️create-mesh must not touch the object's placement");
    assert!(produced.brep.is_none() && produced.properties.is_none(), "🕸️create-mesh must not touch the other two child slots");
}

/// ↩️ With an empty base slot the undo is a single `DeleteMesh`, which clears the slot again.
#[semio_framework_async_macros::async_test]
async fn the_undo_delete_mesh_detaches_the_handle_again() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(
        undo,
        vec![SemioObjectMutation::DeleteMesh(crate::artifacts::semio::standards::v1::subsets::object::schema::mutations::delete_mesh::mutation::DeleteMesh {})],
        "creating a child into an EMPTY slot must undo as the matching delete, not as another create"
    );
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward 🕸️create-mesh applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo DeleteMesh applies to the object that now has a child");
    }
    assert_eq!(current, base, "🕸️create-mesh/attaches-a-mesh-child-to-an-object-that-has-none: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — the payload's own fields are snake_case
/// (`child_id`, no `rename_all` on the struct) while the nested `ArtifactRef`/`ArtifactDialect`
/// are camelCase, and the before-snapshot omits the empty child keys entirely.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioObjectSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "🕸️create-mesh/attaches-a-mesh-child-to-an-object-that-has-none: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("🕸️create-mesh mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("🕸️create-mesh mutation reparses");
    assert_eq!(reencoded, original, "🕸️create-mesh/attaches-a-mesh-child-to-an-object-that-has-none: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the base slot is empty, so the FATAL `mutation.duplicate-id` branch
/// must not fire.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_without_a_duplicate_id_rejection() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "🕸️create-mesh/attaches-a-mesh-child-to-an-object-that-has-none: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "attaching a child to an empty slot must raise no diagnostics");
}

/// 🔺️ The produced delta equals the committed diff — one slot, the handle, nothing else.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioObjectMutation as Mutation<SemioObjectSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "🕸️create-mesh/attaches-a-mesh-child-to-an-object-that-has-none: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical: the inner value is a real handle object, so the
/// `Option<Option<..>>` slot survives decode→encode unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: SemioObjectDiff = serde_json::from_str(DIFF).expect("committed 🕸️create-mesh diff decodes");
    assert!(matches!(decoded.mesh, Some(Some(_))), "the mesh slot must decode as Some(Some(handle)) — set, not cleared");
    assert!(decoded.transform.is_none(), "🕸️create-mesh must leave the transform slot untouched");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "🕸️create-mesh/attaches-a-mesh-child-to-an-object-that-has-none: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioObjectDiff = serde_json::from_str(DIFF).expect("committed 🕸️create-mesh diff decodes");
    let produced = decoded.apply(&before()).expect("committed 🕸️create-mesh diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "🕸️create-mesh/attaches-a-mesh-child-to-an-object-that-has-none: committed diff did not carry before to after");
}
