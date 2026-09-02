//! 🧪️ `delete-primitive` fixture — `removes-the-leading-primitive-and-keeps-the-trailing-one`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: an absent mesh/primitive pair is Error
//! `mutation.target-missing`; otherwise the diff is a nested `primitives.removed[id]` inside a
//! `meshes.modified` entry. The inverse mirrors `delete-mesh`'s dance one level down — strip the
//! trailing primitives, re-create the removed one, rebuild the tail — which is why the fixture
//! removes the LEADING primitive of two.

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> SemioMeshSnapshot {
    serde_json::from_str(BEFORE).expect("delete-primitive before snapshot decodes")
}
fn expected_after() -> SemioMeshSnapshot {
    serde_json::from_str(AFTER).expect("delete-primitive after snapshot decodes")
}
fn mutation() -> SemioMeshMutation {
    serde_json::from_str(MUTATION).expect("delete-primitive mutation decodes")
}

/// ▶️ The leading primitive goes; the trailing one slides down inside the same mesh.
#[semio_framework_async_macros::async_test]
async fn removes_the_leading_primitive_inside_the_mesh() {
    let base = before();
    assert_eq!(base.meshes[0].primitives.len(), 2, "the fixture needs a trailing primitive for the order-restoring inverse to matter");
    let produced = mutation().diff(&base).diff().apply(&base).expect("delete-primitive applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "delete-primitive/removes-the-leading-primitive-and-keeps-the-trailing-one: applied state differs from the committed after-snapshot");
    assert_eq!(produced.meshes.len(), base.meshes.len(), "removing the last-but-one primitive must not remove the mesh itself");
    assert_eq!(produced.meshes[0].primitives, vec![base.meshes[0].primitives[1].clone()], "the trailing primitive slides down into index 0");
}

/// ↩️ The undo is the same three-step dance one level down.
#[semio_framework_async_macros::async_test]
async fn the_undo_strips_the_tail_recreates_the_primitive_then_rebuilds_the_tail() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 3, "one delete per trailing primitive, then the re-create, then one create per trailing primitive");
    assert!(matches!(undo[0], SemioMeshMutation::DeletePrimitive(_)), "the tail is stripped first");
    assert!(matches!(undo[1], SemioMeshMutation::CreatePrimitive(_)) && matches!(undo[2], SemioMeshMutation::CreatePrimitive(_)), "then the removed primitive and the tail are re-created in order");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward delete-primitive applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("each undo step applies to the running state");
    }
    assert_eq!(current, base, "delete-primitive/removes-the-leading-primitive-and-keeps-the-trailing-one: the undo did not restore the before-snapshot, order included");
}

/// 🔣️ Snapshots and the `{"DeletePrimitive":{"mesh_id":…,"primitive_id":…}}` payload are canonical — both payload keys stay snake_case.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioMeshSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-primitive/removes-the-leading-primitive-and-keeps-the-trailing-one: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("delete-primitive mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("delete-primitive mutation reparses");
    assert_eq!(reencoded, original, "delete-primitive/removes-the-leading-primitive-and-keeps-the-trailing-one: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the mesh and the primitive both exist, so mutation.target-missing must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-primitive/removes-the-leading-primitive-and-keeps-the-trailing-one: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "deleting an existing primitive must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. A `meshes.modified` entry whose per-mesh diff carries ONLY the nested `primitives.removed` arm.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioMeshMutation as Mutation<SemioMeshSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-primitive/removes-the-leading-primitive-and-keeps-the-trailing-one: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the collection this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioMeshDiff = serde_json::from_str(DIFF).expect("committed delete-primitive diff decodes");
    let meshes = decoded.meshes.as_ref().expect("delete-primitive must write the meshes triple");
    assert!(meshes.removed.is_empty() && meshes.added.is_empty(), "the mesh itself is modified, never removed");
    let nested = meshes.modified[0].diff.primitives.as_ref().expect("the per-mesh diff must carry a primitives triple");
    assert_eq!(nested.removed, vec!["prim-a".to_string()], "the nested removal is addressed by primitive id");
    assert!(nested.modified.is_empty() && nested.added.is_empty(), "a removal neither modifies nor adds");
    assert!(decoded.materials.is_none() && decoded.textures.is_none(), "no material or texture slot may appear in the diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-primitive/removes-the-leading-primitive-and-keeps-the-trailing-one: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioMeshDiff = serde_json::from_str(DIFF).expect("committed delete-primitive diff decodes");
    let produced = decoded.apply(&before()).expect("committed delete-primitive diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-primitive/removes-the-leading-primitive-and-keeps-the-trailing-one: committed diff did not carry before to after");
}
