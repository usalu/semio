//! 🧪️ `create-primitive` fixture — `adds-a-second-primitive-inside-the-existing-mesh`.
//!
//! Transcribed from `../../🔺️diff/🦀️component.rs`: an unknown `mesh_id` is Error
//! `mutation.target-missing` while a duplicate primitive id inside that mesh is FATAL
//! `mutation.duplicate-id` — two different severities for two different failures. The diff nests
//! TWO levels: a `meshes.modified` entry keyed by mesh id, whose per-mesh diff carries a
//! `primitives` triple with the `NamedAdded` position inside it.

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
    serde_json::from_str(BEFORE).expect("create-primitive before snapshot decodes")
}
fn expected_after() -> SemioMeshSnapshot {
    serde_json::from_str(AFTER).expect("create-primitive after snapshot decodes")
}
fn mutation() -> SemioMeshMutation {
    serde_json::from_str(MUTATION).expect("create-primitive mutation decodes")
}

/// ▶️ The new primitive lands inside the addressed mesh, with its own topology and buffers.
#[semio_framework_async_macros::async_test]
async fn adds_the_line_primitive_inside_the_addressed_mesh() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("create-primitive applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "create-primitive/adds-a-second-primitive-inside-the-existing-mesh: applied state differs from the committed after-snapshot");
    assert_eq!(produced.meshes.len(), base.meshes.len(), "create-primitive may never add a mesh");
    assert_eq!(produced.meshes[0].primitives.len(), base.meshes[0].primitives.len() + 1, "exactly one primitive is added inside the mesh");
    assert_eq!(produced.meshes[0].primitives[1].id, "prim-b", "the new primitive occupies the index the NamedAdded entry recorded");
    assert_eq!(produced.meshes[0].primitives[0], base.meshes[0].primitives[0], "the pre-existing primitive must be byte-identical");
}

/// ↩️ `create-primitive`'s undo is a single `delete-primitive` addressed by mesh AND primitive id.
#[semio_framework_async_macros::async_test]
async fn the_undo_delete_primitive_removes_the_line_primitive_again() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "create-primitive undoes as exactly one delete-primitive");
    let SemioMeshMutation::DeletePrimitive(remove) = &undo[0] else { panic!("create-primitive must undo as delete-primitive") };
    assert_eq!((remove.mesh_id.as_str(), remove.primitive_id.as_str()), ("mesh-a", "prim-b"), "a primitive is addressed by BOTH ids — it has no globally unique key");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward create-primitive applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo delete-primitive applies");
    }
    assert_eq!(current, base, "create-primitive/adds-a-second-primitive-inside-the-existing-mesh: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — the embedded primitive carries `materialId: null` explicitly and its topology encodes as the camelCase `"lines"`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioMeshSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-primitive/adds-a-second-primitive-inside-the-existing-mesh: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("create-primitive mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("create-primitive mutation reparses");
    assert_eq!(reencoded, original, "create-primitive/adds-a-second-primitive-inside-the-existing-mesh: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the mesh exists and the primitive id is free inside it, so neither target-missing nor duplicate-id may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "create-primitive/adds-a-second-primitive-inside-the-existing-mesh: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "creating a primitive with a fresh id inside an existing mesh must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. A `meshes.modified` entry whose per-mesh diff carries ONLY the nested `primitives.added` arm.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioMeshMutation as Mutation<SemioMeshSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-primitive/adds-a-second-primitive-inside-the-existing-mesh: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the collection this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioMeshDiff = serde_json::from_str(DIFF).expect("committed create-primitive diff decodes");
    let meshes = decoded.meshes.as_ref().expect("create-primitive must write the meshes triple");
    assert!(meshes.removed.is_empty() && meshes.added.is_empty(), "the mesh itself is modified, never removed or re-added");
    let nested = meshes.modified[0].diff.primitives.as_ref().expect("the per-mesh diff must carry a primitives triple");
    assert_eq!(nested.added.len(), 1, "exactly one primitive is added");
    assert_eq!(nested.added[0].index, 1, "the nested add carries its target POSITION too");
    assert!(decoded.materials.is_none() && decoded.textures.is_none(), "no material or texture slot may appear in the diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-primitive/adds-a-second-primitive-inside-the-existing-mesh: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioMeshDiff = serde_json::from_str(DIFF).expect("committed create-primitive diff decodes");
    let produced = decoded.apply(&before()).expect("committed create-primitive diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-primitive/adds-a-second-primitive-inside-the-existing-mesh: committed diff did not carry before to after");
}
