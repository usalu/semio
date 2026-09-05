//! 🧪️ `create-mesh` fixture — `🐼️adds-an-empty-second-mesh-at-the-end`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs` and the `diff_add_mesh` helper it delegates to:
//! a duplicate mesh id is FATAL `mutation.duplicate-id`; otherwise the diff is a `meshes` triple
//! whose `added` entry is a `NamedAdded { index, item }` — this subset carries a POSITION alongside
//! the id-keyed add (a local fix over the shared engine, which could only ever append), and the
//! index is `base.meshes.len()`, i.e. the end.

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
    serde_json::from_str(BEFORE).expect("create-mesh before snapshot decodes")
}
fn expected_after() -> SemioMeshSnapshot {
    serde_json::from_str(AFTER).expect("create-mesh after snapshot decodes")
}
fn mutation() -> SemioMeshMutation {
    serde_json::from_str(MUTATION).expect("create-mesh mutation decodes")
}

/// ▶️ The new mesh lands at the index the diff named, and it starts with no primitives.
#[semio_framework_async_macros::async_test]
async fn adds_the_empty_mesh_at_the_recorded_index() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("create-mesh applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "create-mesh/adds-an-empty-second-mesh-at-the-end: applied state differs from the committed after-snapshot");
    assert_eq!(produced.meshes.len(), base.meshes.len() + 1, "create-mesh adds exactly one mesh");
    assert_eq!(produced.meshes[1].id, "mesh-b", "the new mesh occupies the index the NamedAdded entry recorded");
    assert!(produced.meshes[1].primitives.is_empty(), "the payload carried an empty primitive list and it must land empty");
    assert_eq!(produced.meshes[0], base.meshes[0], "the pre-existing mesh must be byte-identical");
    assert_eq!((produced.materials, produced.textures), (base.materials, base.textures), "creating a mesh touches neither materials nor textures");
}

/// ↩️ `create-mesh`'s undo is a single `delete-mesh` for the same id.
#[semio_framework_async_macros::async_test]
async fn the_undo_delete_mesh_removes_the_second_mesh_again() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "create-mesh undoes as exactly one delete-mesh");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward create-mesh applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo delete-mesh applies");
    }
    assert_eq!(current, base, "create-mesh/adds-an-empty-second-mesh-at-the-end: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — a primitive always emits `materialId` (no `skip_serializing_if`, so an unbound primitive shows an explicit `null`) and `SemioTopology` is camelCase (`triangleStrip`, not `TriangleStrip`).
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioMeshSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-mesh/adds-an-empty-second-mesh-at-the-end: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("create-mesh mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("create-mesh mutation reparses");
    assert_eq!(reencoded, original, "create-mesh/adds-an-empty-second-mesh-at-the-end: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: no mesh with id mesh-b exists, so the FATAL mutation.duplicate-id branch must not fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "create-mesh/adds-an-empty-second-mesh-at-the-end: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "creating a mesh with a fresh id must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. Only the `meshes` triple, and only its `added` arm.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioMeshMutation as Mutation<SemioMeshSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-mesh/adds-an-empty-second-mesh-at-the-end: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the collection this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioMeshDiff = serde_json::from_str(DIFF).expect("committed create-mesh diff decodes");
    let meshes = decoded.meshes.as_ref().expect("create-mesh must write the meshes triple");
    assert_eq!(meshes.added.len(), 1, "exactly one mesh is added");
    assert_eq!(meshes.added[0].index, 1, "the add carries its target POSITION, not just the item");
    assert!(meshes.removed.is_empty() && meshes.modified.is_empty(), "a create neither removes nor modifies");
    assert!(decoded.materials.is_none() && decoded.textures.is_none(), "no material or texture slot may appear in the diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-mesh/adds-an-empty-second-mesh-at-the-end: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioMeshDiff = serde_json::from_str(DIFF).expect("committed create-mesh diff decodes");
    let produced = decoded.apply(&before()).expect("committed create-mesh diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-mesh/adds-an-empty-second-mesh-at-the-end: committed diff did not carry before to after");
}
