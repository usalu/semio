//! 🧪️ `move-vertex` fixture — `lifts-the-third-vertex-of-the-triangle`.
//!
//! Transcribed from `../../🔺️diff/🦀️component.rs`, guards in order: absent mesh/primitive ⇒ Error
//! `mutation.target-missing`; `vertex_index` past the end ⇒ the same Error; an unchanged point ⇒
//! Warning `mutation.no-op`; a non-finite component ⇒ FATAL `mutation.invariant`. All four address
//! themselves as `"<mesh>:<primitive>:<index>"`. Because `SemioPrimitiveDiff::positions` can only
//! express a WHOLE-array replace, `diff_move_vertex` clones BASE's positions and patches just the
//! one slot — so the committed diff legitimately carries every vertex, changed or not, and that is
//! the shape being pinned here.

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
    serde_json::from_str(BEFORE).expect("move-vertex before snapshot decodes")
}
fn expected_after() -> SemioMeshSnapshot {
    serde_json::from_str(AFTER).expect("move-vertex after snapshot decodes")
}
fn mutation() -> SemioMeshMutation {
    serde_json::from_str(MUTATION).expect("move-vertex mutation decodes")
}

/// ▶️ Exactly one vertex moves; the other two and every parallel buffer stay put.
#[semio_framework_async_macros::async_test]
async fn moves_exactly_one_vertex_of_the_position_buffer() {
    let base = before();
    let produced = mutation().diff(&base).diff().apply(&base).expect("move-vertex applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "move-vertex/lifts-the-third-vertex-of-the-triangle: applied state differs from the committed after-snapshot");
    let edited = &produced.meshes[0].primitives[0];
    let original = &base.meshes[0].primitives[0];
    assert_eq!(edited.positions.len(), original.positions.len(), "moving a vertex may never resize the buffer");
    assert_eq!(edited.positions[2].z, 0.5, "the addressed vertex takes the payload's absolute point");
    assert_eq!(&edited.positions[..2], &original.positions[..2], "the untargeted vertices must be byte-identical");
    assert_eq!(edited.indices, original.indices, "moving a vertex must not renumber the index buffer");
}

/// ↩️ The undo is a `move-vertex` carrying BASE's captured point for that index.
#[semio_framework_async_macros::async_test]
async fn the_undo_move_vertex_restores_the_captured_point() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "move-vertex undoes as exactly one move-vertex");
    let SemioMeshMutation::MoveVertex(restore) = &undo[0] else { panic!("move-vertex must undo as itself") };
    assert_eq!(restore.vertex_index, 2, "the undo addresses the same vertex index");
    assert_eq!(restore.new_point, base.meshes[0].primitives[0].positions[2], "and carries BASE's own point for it");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward move-vertex applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo move-vertex applies");
    }
    assert_eq!(current, base, "move-vertex/lifts-the-third-vertex-of-the-triangle: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the payload are canonical — every coordinate is dyadic so decode→encode is exact.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioMeshSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "move-vertex/lifts-the-third-vertex-of-the-triangle: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("move-vertex mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("move-vertex mutation reparses");
    assert_eq!(reencoded, original, "move-vertex/lifts-the-third-vertex-of-the-triangle: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the primitive and the vertex index both exist, the point is finite and genuinely different, so none of the four guards may fire
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_as_committed() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "move-vertex/lifts-the-third-vertex-of-the-triangle: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "a finite, in-range, genuinely-different vertex move must raise no diagnostics");
}

/// 🔺️ The sparse delta this mutation produces equals the committed diff. A doubly-nested `primitives.modified` entry carrying `positions` alone — the WHOLE array, since the diff type has no per-vertex slot.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioMeshMutation as Mutation<SemioMeshSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "move-vertex/lifts-the-third-vertex-of-the-triangle: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is a decode→encode fixed point, and only the collection this mutation is
/// allowed to touch appears in it at all.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical_and_narrowly_scoped() {
    let decoded: SemioMeshDiff = serde_json::from_str(DIFF).expect("committed move-vertex diff decodes");
    let meshes = decoded.meshes.as_ref().expect("the meshes triple must be present");
    assert!(meshes.removed.is_empty() && meshes.added.is_empty(), "the mesh is modified, never removed or re-added");
    let nested = meshes.modified[0].diff.primitives.as_ref().expect("the per-mesh diff must carry a primitives triple");
    assert!(nested.removed.is_empty() && nested.added.is_empty(), "the primitive is modified per field, never removed and re-added");
    let pdiff = &nested.modified[0].diff;
    let positions = pdiff.positions.as_ref().expect("the positions buffer must be written");
    assert_eq!(positions.len(), 3, "the diff carries the whole rebuilt position buffer, not just the moved vertex");
    assert_eq!(positions[0], before().meshes[0].primitives[0].positions[0], "and the untargeted vertices in it are BASE's own");
    assert!(pdiff.normals.is_none() && pdiff.uvs.is_none() && pdiff.colors.is_none() && pdiff.indices.is_none(), "no other buffer may be written");
    assert!(pdiff.topology.is_none() && pdiff.material_id.is_none(), "neither the draw mode nor the material binding may be written");
    assert!(decoded.materials.is_none() && decoded.textures.is_none(), "no material or texture slot may appear in the diff");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "move-vertex/lifts-the-third-vertex-of-the-triangle: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: SemioMeshDiff = serde_json::from_str(DIFF).expect("committed move-vertex diff decodes");
    let produced = decoded.apply(&before()).expect("committed move-vertex diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "move-vertex/lifts-the-third-vertex-of-the-triangle: committed diff did not carry before to after");
}
