//! 🧪️ `delete-mesh` fixture — `removes-the-leading-mesh-and-keeps-the-trailing-one`.
//!
//! Transcribed from `../../🔺️diff/🦀️component.rs`: an unknown mesh id is Error
//! `mutation.target-missing`; otherwise the diff is a bare `meshes.removed[id]` that carries no
//! content. `↩️inverse/🦀️component.rs` is the interesting half: because the id-keyed add can only
//! re-insert at a recorded index, the inverse first DELETES every mesh after the removed one, then
//! re-creates the removed mesh, then re-creates that tail — so order is restored exactly. Removing
//! the LEADING mesh of two is what makes that dance observable.

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::diff::SemioMeshDiff;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> SemioMeshSnapshot {
    serde_json::from_str(BEFORE).expect("delete-mesh before snapshot decodes")
}
fn expected_after() -> SemioMeshSnapshot {
    serde_json::from_str(AFTER).expect("delete-mesh after snapshot decodes")
}
fn mutation() -> SemioMeshMutation {
    serde_json::from_str(MUTATION).expect("delete-mesh mutation decodes")
}

/// ▶️ The leading mesh goes; the trailing one slides down to index 0.
#[semio_framework_async_macros::async_test]
async fn removes_the_leading_mesh() {
    let base = before();
    assert_eq!(base.meshes.len(), 2, "the fixture needs a trailing mesh for the order-restoring inverse to matter");
    let produced = mutation().diff(&base).diff().apply(&base).expect("delete-mesh applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "delete-mesh/removes-the-leading-mesh-and-keeps-the-trailing-one: applied state differs from the committed after-snapshot");
    assert!(!produced.meshes.iter().any(|mesh| mesh.id == "mesh-a"), "the named mesh must be gone");
    assert_eq!(produced.meshes, vec![base.meshes[1].clone()], "the trailing mesh slides down into index 0");
    assert_eq!(produced.materials, base.materials, "deleting a mesh must not cascade into the materials its primitives referenced");
}

/// ↩️ The undo is a THREE-step dance: strip the tail, re-create the removed mesh, rebuild the tail.
#[semio_framework_async_macros::async_test]
async fn the_undo_strips_the_tail_recreates_the_mesh_then_rebuilds_the_tail() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 3, "one delete per trailing mesh, then the re-create, then one create per trailing mesh");
    assert!(matches!(undo[0], SemioMeshMutation::DeleteMesh(_)), "the tail is stripped first so the removed mesh can be re-inserted ahead of it");
    assert!(matches!(undo[1], SemioMeshMutation::CreateMesh(_)) && matches!(undo[2], SemioMeshMutation::CreateMesh(_)), "then the removed mesh and the tail are re-created in order");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward delete-mesh applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("each undo step applies to the running state");
    }
    assert_eq!(current, base, "delete-mesh/removes-the-leading-mesh-and-keeps-the-trailing-one: the undo did not restore the before-snapshot, order included");
}

/// 🔣️ Snapshots and the `{"DeleteMesh":{"id":"mesh-a"}}` payload are canonical fixed points.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioMeshSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-mesh/removes-the-leading-mesh-and-keeps-the-trailing-one: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("delete-mesh mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("delete-mesh mutation reparses");
    assert_eq!(reencoded, original, "delete-mesh/removes-the-leading-mesh-and-keeps-the-trailing-one: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the mesh exists, so mutation.target-missing must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-mesh/removes-the-leading-mesh-and-keeps-the-trailing-one: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "deleting an existing mesh must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only `meshes.removed`, carrying the ID — a delete diff never carries the deleted content.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioMeshMutation as Mutation<SemioMeshSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-mesh/removes-the-leading-mesh-and-keeps-the-trailing-one: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the collection this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioMeshDiff = serde_json::from_str(DIFF).expect("committed delete-mesh diff decodes");
    let meshes = decoded.meshes.as_ref().expect("delete-mesh must write the meshes triple");
    assert_eq!(meshes.removed, vec!["mesh-a".to_string()], "the removal is addressed by mesh id");
    assert!(meshes.modified.is_empty() && meshes.added.is_empty(), "a removal neither modifies nor adds");
    assert!(decoded.materials.is_none() && decoded.textures.is_none(), "no material or texture slot may appear in the diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-mesh/removes-the-leading-mesh-and-keeps-the-trailing-one: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioMeshDiff = serde_json::from_str(DIFF).expect("committed delete-mesh diff decodes");
    let produced = decoded.apply(&before()).expect("committed delete-mesh diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-mesh/removes-the-leading-mesh-and-keeps-the-trailing-one: committed diff did not carry before to after");
}
